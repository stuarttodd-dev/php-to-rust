mod dto;
mod resources;
mod validation;

use crate::domain::Product;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use dto::{AddBasketItemBody, PatchBasketItemBody};
use resources::ProductResponse;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use axum::routing::{patch, post};
use serde::Serialize;
use sqlx::Row;
use validation::{validate_add_item, validate_patch_qty};

/// All HTTP routes on one `Router` (no `nest("/api", …)`).
/// Nested routers can interact oddly with path matching in some Axum versions; flat routes are predictable.
pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/products", get(list_products))
        .route("/api/products/{id}", get(get_product))
        .route(
            "/api/baskets/{basket_id}/items/{product_id}",
            patch(patch_basket_line).delete(delete_basket_line),
        )
        .route("/api/baskets/{basket_id}/items", post(post_basket_item))
        .route(
            "/api/baskets/{basket_id}",
            get(get_basket).delete(clear_basket),
        )
        .route("/", get(root))
        .with_state(pool)
}

async fn list_products(State(pool): State<Arc<PgPool>>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Product>(
        "SELECT id, title, price_cents, cost_cents FROM products ORDER BY id",
    )
    .fetch_all(&*pool)
    .await;

    match result {
        Ok(rows) => {
            match rows
                .into_iter()
                .map(|p| Product::try_new(p.id, p.title, p.price_cents, p.cost_cents))
                .collect::<Result<Vec<_>, _>>()
            {
                Ok(products) => {
                    let list: Vec<ProductResponse> =
                        products.iter().map(ProductResponse::from).collect();
                    (StatusCode::OK, Json(list)).into_response()
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": e.to_string() })),
                )
                    .into_response(),
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_product(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, Product>(
        "SELECT id, title, price_cents, cost_cents FROM products WHERE id = $1",
    )
    .bind(id as i64)
    .fetch_optional(&*pool)
    .await;

    match result {
        Ok(Some(p)) => match Product::try_new(p.id, p.title, p.price_cents, p.cost_cents) {
            Ok(product) => {
                let response = ProductResponse::from(&product);
                (StatusCode::OK, Json(response)).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response(),
        },
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "product not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn health(State(pool): State<Arc<PgPool>>) -> impl axum::response::IntoResponse {
    match sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&*pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "OK", "service": "shop-api" }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}

async fn root() -> impl axum::response::IntoResponse {
    (StatusCode::OK, Json(json!({
        "message": "Shop API with PostgreSQL",
        "endpoints": [
            "/api/health — liveness (DB ping)",
            "/api/products — catalog JSON",
            "/api/products/{id} — one product by id",
            "/api/baskets/{basket_id} — GET basket lines, DELETE clear",
            "/api/baskets/{basket_id}/items — POST add line (JSON: product_id, qty)",
            "/api/baskets/{basket_id}/items/{product_id} — PATCH qty, DELETE line",
            "/ — this message"
        ]
    })))
}

fn normalize_basket_id(basket_id: String) -> Result<String, axum::response::Response> {
    let s = basket_id.trim().to_string();
    if s.is_empty() {
        Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "invalid basket id" })),
        )
            .into_response())
    } else {
        Ok(s)
    }
}

/// Laravel-style validation error for `qty` (stable JSON contract).
fn qty_validation_json_response() -> axum::response::Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({
            "error": "qty must be greater than zero",
            "field": "qty",
        })),
    )
        .into_response()
}

async fn post_basket_item(
    State(pool): State<Arc<PgPool>>,
    Path(basket_id): Path<String>,
    Json(body): Json<AddBasketItemBody>,
) -> impl IntoResponse {
    let basket_id = match normalize_basket_id(basket_id) {
        Ok(s) => s,
        Err(r) => return r,
    };

    if validate_add_item(&body).is_err() {
        return qty_validation_json_response();
    }

    let ok = sqlx::query_scalar::<_, i32>("SELECT 1 FROM products WHERE id = $1")
        .bind(body.product_id as i64)
        .fetch_optional(&*pool)
        .await;

    match ok {
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "product not found" })),
            )
                .into_response();
        }
        Ok(Some(_)) => {}
    }

    match sqlx::query(
        r#"
        INSERT INTO basket_lines (basket_id, product_id, qty)
        VALUES ($1, $2, $3)
        ON CONFLICT (basket_id, product_id)
        DO UPDATE SET qty = basket_lines.qty + EXCLUDED.qty
        "#,
    )
    .bind(&basket_id)
    .bind(body.product_id as i64)
    .bind(body.qty as i32)
    .execute(&*pool)
    .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

#[derive(Serialize)]
struct BasketLineJson {
    product_id: u64,
    title: String,
    quantity: u32,
    line_subtotal_cents: u64,
}

#[derive(Serialize)]
struct BasketJson {
    lines: Vec<BasketLineJson>,
    subtotal_cents: u64,
}

async fn get_basket(
    State(pool): State<Arc<PgPool>>,
    Path(basket_id): Path<String>,
) -> impl IntoResponse {
    let basket_id = match normalize_basket_id(basket_id) {
        Ok(s) => s,
        Err(r) => return r,
    };

    let rows = match sqlx::query(
        r#"
        SELECT cl.product_id, p.title, cl.qty, p.price_cents
        FROM basket_lines cl
        INNER JOIN products p ON p.id = cl.product_id
        WHERE cl.basket_id = $1
        ORDER BY cl.product_id
        "#,
    )
    .bind(&basket_id)
    .fetch_all(&*pool)
    .await
    {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
                .into_response();
        }
    };

    let mut lines = Vec::new();
    let mut subtotal: u64 = 0;
    for row in rows {
        let product_id: i64 = row.try_get("product_id").unwrap_or(0);
        let title: String = row.try_get("title").unwrap_or_default();
        let qty: i32 = row.try_get("qty").unwrap_or(0);
        let price_cents: i32 = row.try_get("price_cents").unwrap_or(0);
        let q = qty as u32;
        let p = price_cents as u64;
        let line_subtotal = p * q as u64;
        subtotal += line_subtotal;
        lines.push(BasketLineJson {
            product_id: product_id as u64,
            title,
            quantity: q,
            line_subtotal_cents: line_subtotal,
        });
    }

    (StatusCode::OK, Json(BasketJson { lines, subtotal_cents: subtotal })).into_response()
}

async fn patch_basket_line(
    State(pool): State<Arc<PgPool>>,
    Path((basket_id, product_id)): Path<(String, u64)>,
    Json(body): Json<PatchBasketItemBody>,
) -> impl IntoResponse {
    let basket_id = match normalize_basket_id(basket_id) {
        Ok(s) => s,
        Err(r) => return r,
    };

    if validate_patch_qty(&body).is_err() {
        return qty_validation_json_response();
    }

    match sqlx::query(
        "UPDATE basket_lines SET qty = $3 WHERE basket_id = $1 AND product_id = $2",
    )
    .bind(&basket_id)
    .bind(product_id as i64)
    .bind(body.qty as i32)
    .execute(&*pool)
    .await
    {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "basket line not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn delete_basket_line(
    State(pool): State<Arc<PgPool>>,
    Path((basket_id, product_id)): Path<(String, u64)>,
) -> impl IntoResponse {
    let basket_id = match normalize_basket_id(basket_id) {
        Ok(s) => s,
        Err(r) => return r,
    };

    match sqlx::query("DELETE FROM basket_lines WHERE basket_id = $1 AND product_id = $2")
        .bind(&basket_id)
        .bind(product_id as i64)
        .execute(&*pool)
        .await
    {
        Ok(r) if r.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "basket line not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn clear_basket(
    State(pool): State<Arc<PgPool>>,
    Path(basket_id): Path<String>,
) -> impl IntoResponse {
    let basket_id = match normalize_basket_id(basket_id) {
        Ok(s) => s,
        Err(r) => return r,
    };

    match sqlx::query("DELETE FROM basket_lines WHERE basket_id = $1")
        .bind(&basket_id)
        .execute(&*pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}


