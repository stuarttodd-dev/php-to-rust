use axum::Json;
use axum::Router;
use axum::extract::{Path, State};
use axum::http::{HeaderValue, Method, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Basket(#[from] shop::BasketError),
    #[error("product not found")]
    MissingProduct,
    #[error("basket not found")]
    MissingBasket,
    #[error("unknown code")]
    UnknownCode,
    #[error("database error")]
    Database(#[from] sqlx::Error),
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::Basket(shop::BasketError::ZeroQuantity | shop::BasketError::Overflow) => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            AppError::MissingProduct | AppError::MissingBasket => StatusCode::NOT_FOUND,
            AppError::UnknownCode => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Database(err) => {
                tracing::error!(?err, "database error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let message = match &self {
            AppError::Database(_) => "internal error".to_string(),
            other => other.to_string(),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

#[derive(Serialize)]
struct CreatedBasket {
    id: i32,
}

#[derive(Deserialize, Serialize)]
struct NewLine {
    product_id: u64,
    #[serde(rename = "qty")]
    quantity: u32,
}

#[derive(Serialize)]
struct LineView {
    product_id: u64,
    #[serde(rename = "qty")]
    quantity: u32,
}

#[derive(Serialize)]
struct BasketView {
    lines: Vec<LineView>,
    total_cents: u32,
}

#[derive(Serialize)]
struct Receipt {
    total_cents: u32,
}

#[derive(Deserialize)]
struct DiscountBody {
    code: String,
}

#[derive(Serialize)]
struct ProductView {
    id: i32,
    title: String,
    price_cents: i32,
}

struct ProductRow {
    id: i32,
    title: String,
    price_cents: i32,
}

struct LineRow {
    product_id: i32,
    quantity: i32,
}

pub fn app(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1:8091".parse::<HeaderValue>().expect("origin"))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);
    Router::new()
        .route("/health", get(health))
        .route("/products/{id}", get(product))
        .route("/products", get(products))
        .route("/baskets/{id}/checkout", post(checkout))
        .route("/baskets/{id}/discount", post(apply_discount))
        .route("/baskets/{id}/lines", post(add_line))
        .route("/baskets/{id}", get(show_basket))
        .route("/baskets", post(create_basket))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}

async fn health() -> &'static str {
    "ok"
}

async fn products(State(pool): State<PgPool>) -> Result<Json<Vec<ProductView>>, AppError> {
    let rows = sqlx::query_as!(
        ProductRow,
        "SELECT id, title, price_cents FROM products ORDER BY id"
    )
    .fetch_all(&pool)
    .await?;
    Ok(Json(
        rows.into_iter()
            .map(|row| ProductView {
                id: row.id,
                title: row.title,
                price_cents: row.price_cents,
            })
            .collect(),
    ))
}

async fn product(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<ProductView>, AppError> {
    let row = sqlx::query_as!(
        ProductRow,
        "SELECT id, title, price_cents FROM products WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?;
    row.map(|row| {
        Json(ProductView {
            id: row.id,
            title: row.title,
            price_cents: row.price_cents,
        })
    })
    .ok_or(AppError::MissingProduct)
}

async fn create_basket(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<CreatedBasket>), AppError> {
    let id = sqlx::query_scalar!("INSERT INTO baskets DEFAULT VALUES RETURNING id")
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::CREATED, Json(CreatedBasket { id })))
}

async fn add_line(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(line): Json<NewLine>,
) -> Result<(StatusCode, Json<NewLine>), AppError> {
    if line.quantity == 0 {
        return Err(shop::BasketError::ZeroQuantity.into());
    }
    let product_id = i32::try_from(line.product_id).map_err(|_| AppError::MissingProduct)?;
    let quantity =
        i32::try_from(line.quantity).map_err(|_| AppError::Basket(shop::BasketError::Overflow))?;
    let mut tx = pool.begin().await?;
    lock_basket(&mut tx, id).await?;
    load_product(&pool, product_id).await?;
    sqlx::query!(
        "INSERT INTO basket_lines (basket_id, product_id, quantity) VALUES ($1, $2, $3)
         ON CONFLICT (basket_id, product_id)
         DO UPDATE SET quantity = basket_lines.quantity + EXCLUDED.quantity",
        id,
        product_id,
        quantity,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(line)))
}

async fn show_basket(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<BasketView>, AppError> {
    load_basket(&pool, id).await?;
    let basket = load_lines(&pool, id).await?;
    let catalog = load_catalog(&pool).await?;
    let total = basket.total(&catalog)?;
    let mut lines: Vec<LineView> = basket
        .lines
        .iter()
        .map(|line| LineView {
            product_id: line.product_id,
            quantity: line.quantity,
        })
        .collect();
    lines.sort_by_key(|line| line.product_id);
    Ok(Json(BasketView {
        lines,
        total_cents: total.0,
    }))
}

async fn checkout(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Receipt>), AppError> {
    let mut tx = pool.begin().await?;
    lock_basket(&mut tx, id).await?;
    let basket = load_lines_tx(&mut tx, id).await?;
    let catalog = load_catalog(&pool).await?;
    let total_cents = match basket.checkout(&catalog)? {
        shop::Order::CheckedOut { total } => total.0,
        shop::Order::Open(_) => return Err(AppError::MissingBasket),
    };
    // Schema CHECK and non-negative domain totals keep this conversion in range.
    let total_i32 =
        i32::try_from(total_cents).map_err(|_| AppError::Basket(shop::BasketError::Overflow))?;
    sqlx::query!("INSERT INTO orders (total_cents) VALUES ($1)", total_i32)
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM basket_lines WHERE basket_id = $1", id)
        .execute(&mut *tx)
        .await?;
    sqlx::query!("DELETE FROM baskets WHERE id = $1", id)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok((StatusCode::OK, Json(Receipt { total_cents })))
}

async fn apply_discount(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(body): Json<DiscountBody>,
) -> Result<(StatusCode, Json<Receipt>), AppError> {
    let _ = (pool, id, body.code);
    todo!()
}

async fn lock_basket(tx: &mut Transaction<'_, Postgres>, id: i32) -> Result<(), AppError> {
    let found = sqlx::query_scalar!("SELECT id FROM baskets WHERE id = $1 FOR UPDATE", id)
        .fetch_optional(&mut **tx)
        .await?;
    if found.is_none() {
        return Err(AppError::MissingBasket);
    }
    Ok(())
}

async fn load_basket(pool: &PgPool, id: i32) -> Result<(), AppError> {
    let found = sqlx::query_scalar!("SELECT id FROM baskets WHERE id = $1", id)
        .fetch_optional(pool)
        .await?;
    if found.is_none() {
        return Err(AppError::MissingBasket);
    }
    Ok(())
}

async fn load_product(pool: &PgPool, id: i32) -> Result<shop::Product, AppError> {
    let row = sqlx::query_as!(
        ProductRow,
        "SELECT id, title, price_cents FROM products WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;
    row.map(to_product).ok_or(AppError::MissingProduct)
}

async fn load_lines(pool: &PgPool, id: i32) -> Result<shop::Basket, AppError> {
    let rows = sqlx::query_as!(
        LineRow,
        "SELECT product_id, quantity FROM basket_lines WHERE basket_id = $1 ORDER BY product_id",
        id
    )
    .fetch_all(pool)
    .await?;
    Ok(shop::Basket {
        lines: rows.iter().map(to_line).collect(),
    })
}

async fn load_lines_tx(
    tx: &mut Transaction<'_, Postgres>,
    id: i32,
) -> Result<shop::Basket, AppError> {
    let rows = sqlx::query_as!(
        LineRow,
        "SELECT product_id, quantity FROM basket_lines WHERE basket_id = $1 ORDER BY product_id",
        id
    )
    .fetch_all(&mut **tx)
    .await?;
    Ok(shop::Basket {
        lines: rows.iter().map(to_line).collect(),
    })
}

async fn load_catalog(pool: &PgPool) -> Result<Vec<shop::Product>, AppError> {
    let rows = sqlx::query_as!(
        ProductRow,
        "SELECT id, title, price_cents FROM products ORDER BY id"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(to_product).collect())
}

fn to_product(row: ProductRow) -> shop::Product {
    // CHECK (price_cents >= 0) and positive product ids keep these conversions valid.
    shop::Product {
        id: u64::try_from(row.id).expect("product id fits u64"),
        title: row.title,
        price_cents: u32::try_from(row.price_cents).expect("price_cents fits u32"),
    }
}

fn to_line(row: &LineRow) -> shop::BasketLine {
    // CHECK (quantity > 0) keeps quantity in range for u32.
    shop::BasketLine {
        product_id: u64::try_from(row.product_id).expect("product_id fits u64"),
        quantity: u32::try_from(row.quantity).expect("quantity fits u32"),
    }
}
