//! Product reviews API (Laravel-style capstone).
//!
//! HTTP handlers use an in-memory store (product ids `1`, `2`, and `3`) so `oneshot`
//! tests need no database. `tests/sqlx_reviews.rs` uses `#[sqlx::test]` against
//! Postgres when `DATABASE_URL` is set (CI provides Postgres 16).

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, RwLock};

use axum::Json;
use axum::Router;
use axum::extract::{Path, Query, State};
use axum::http::{Request, StatusCode, header::AUTHORIZATION};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::handler::Handler;
use axum::routing::get;
use serde::{Deserialize, Serialize};

/// Expected `Authorization` header value for protected routes: `Bearer secret`.
pub const BEARER_TOKEN: &str = "secret";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    MissingProduct,
    Unauthorized,
    Validation(&'static str),
}

#[derive(Serialize, Deserialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::MissingProduct => (StatusCode::NOT_FOUND, "product not found".to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.to_string()),
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Review {
    pub id: i32,
    pub product_id: i64,
    pub rating: i32,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewReview {
    pub rating: i32,
    pub body: String,
}

impl NewReview {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !(1..=5).contains(&self.rating) {
            return Err("rating must be between 1 and 5");
        }
        if self.body.is_empty() || self.body.len() > 500 {
            return Err("body must be 1 to 500 characters");
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct Store {
    reviews: RwLock<Vec<Review>>,
    next_id: AtomicI32,
}

impl Store {
    fn insert(&self, product_id: i64, new_review: NewReview) -> Review {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let review = Review {
            id,
            product_id,
            rating: new_review.rating,
            body: new_review.body,
        };
        self.reviews.write().expect("reviews lock").push(review.clone());
        review
    }

    fn for_product(&self, product_id: i64) -> Vec<Review> {
        self.reviews
            .read()
            .expect("reviews lock")
            .iter()
            .filter(|r| r.product_id == product_id)
            .cloned()
            .collect()
    }
}

#[derive(Clone)]
pub struct AppState {
    store: Arc<Store>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            store: Arc::new(Store::default()),
        }
    }

    pub fn with_reviews(reviews: Vec<Review>) -> Self {
        let next_id = reviews.iter().map(|r| r.id).max().unwrap_or(0) + 1;
        Self {
            store: Arc::new(Store {
                reviews: RwLock::new(reviews),
                next_id: AtomicI32::new(next_id),
            }),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Deserialize)]
struct PaginationQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    10
}

#[derive(Serialize, Deserialize)]
struct PaginationMeta {
    current_page: u64,
    per_page: u64,
    total: u64,
}

#[derive(Serialize, Deserialize)]
struct PaginatedReviews {
    data: Vec<Review>,
    meta: PaginationMeta,
}

pub fn app() -> Router {
    app_with_state(AppState::new())
}

pub fn app_with_state(state: AppState) -> Router {
    Router::new()
        .route(
            "/products/{product_id}/reviews",
            get(list_reviews).post(create_review.layer(middleware::from_fn(require_bearer))),
        )
        .with_state(state)
}

fn product_exists(product_id: i64) -> bool {
    (1..=3).contains(&product_id)
}

async fn require_bearer(request: Request<axum::body::Body>, next: Next) -> Response {
    let authorized = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|header| header.strip_prefix("Bearer ") == Some(BEARER_TOKEN));
    if !authorized {
        return AppError::Unauthorized.into_response();
    }
    next.run(request).await
}

async fn list_reviews(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedReviews>, AppError> {
    if !product_exists(product_id) {
        return Err(AppError::MissingProduct);
    }

    let page = query.page.max(1);
    let per_page = query.per_page.max(1);
    let all = state.store.for_product(product_id);
    let total = all.len() as u64;

    let offset = (page - 1).saturating_mul(per_page) as usize;
    let end = offset.saturating_add(per_page as usize).min(all.len());
    let data = if offset >= all.len() {
        Vec::new()
    } else {
        all[offset..end].to_vec()
    };

    Ok(Json(PaginatedReviews {
        data,
        meta: PaginationMeta {
            current_page: page,
            per_page,
            total,
        },
    }))
}

async fn create_review(
    State(state): State<AppState>,
    Path(product_id): Path<i64>,
    Json(body): Json<NewReview>,
) -> Result<(StatusCode, Json<Review>), AppError> {
    if !product_exists(product_id) {
        return Err(AppError::MissingProduct);
    }
    body.validate().map_err(AppError::Validation)?;
    let review = state.store.insert(product_id, body);
    Ok((StatusCode::CREATED, Json(review)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn auth_header() -> (&'static str, &'static str) {
        ("authorization", "Bearer secret")
    }

    #[test]
    fn rejects_bad_rating() {
        let review = NewReview {
            rating: 6,
            body: String::from("too hot"),
        };
        assert_eq!(review.validate(), Err("rating must be between 1 and 5"));
    }

    #[test]
    fn rejects_empty_body() {
        let review = NewReview {
            rating: 5,
            body: String::from(""),
        };
        assert_eq!(
            review.validate(),
            Err("body must be 1 to 500 characters")
        );
    }

    #[tokio::test]
    async fn post_without_token_returns_401() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/products/1/reviews")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"rating":5,"body":"nice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn post_with_wrong_token_returns_401() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/products/1/reviews")
                    .header("content-type", "application/json")
                    .header("authorization", "Bearer wrong")
                    .body(Body::from(r#"{"rating":5,"body":"nice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn post_bad_body_returns_422() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/products/1/reviews")
                    .header("content-type", "application/json")
                    .header(auth_header().0, auth_header().1)
                    .body(Body::from(r#"{"rating":9,"body":"nice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let body: ErrorBody = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(body.error, "rating must be between 1 and 5");
    }

    #[tokio::test]
    async fn get_missing_product_returns_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/products/99/reviews")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn post_missing_product_returns_404() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/products/99/reviews")
                    .header("content-type", "application/json")
                    .header(auth_header().0, auth_header().1)
                    .body(Body::from(r#"{"rating":5,"body":"nice"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn get_list_pagination_returns_200() {
        let mut reviews = Vec::new();
        for i in 0..15 {
            reviews.push(Review {
                id: i + 1,
                product_id: 1,
                rating: 4,
                body: format!("review {i}"),
            });
        }
        let router = app_with_state(AppState::with_reviews(reviews));

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/products/1/reviews?page=2&per_page=5")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let page: PaginatedReviews = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(page.data.len(), 5);
        assert_eq!(page.data[0].body, "review 5");
        assert_eq!(page.meta.current_page, 2);
        assert_eq!(page.meta.per_page, 5);
        assert_eq!(page.meta.total, 15);
    }

    #[tokio::test]
    async fn get_list_default_pagination() {
        let router = app_with_state(AppState::with_reviews(vec![Review {
            id: 1,
            product_id: 2,
            rating: 5,
            body: "great".into(),
        }]));

        let response = router
            .oneshot(
                Request::builder()
                    .uri("/products/2/reviews")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let page: PaginatedReviews = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(page.data.len(), 1);
        assert_eq!(page.meta.current_page, 1);
        assert_eq!(page.meta.per_page, 10);
        assert_eq!(page.meta.total, 1);
    }

    #[tokio::test]
    async fn post_valid_review_returns_201() {
        let response = app()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/products/3/reviews")
                    .header("content-type", "application/json")
                    .header(auth_header().0, auth_header().1)
                    .body(Body::from(r#"{"rating":5,"body":"excellent"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::CREATED);
    }
}
