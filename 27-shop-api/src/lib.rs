//! Shop API (catalog + basket): domain rules, HTTP adapters (SQLx in handlers for this chapter).

pub mod api;
pub mod domain;

use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;

/// HTTP router: **`GET /`**, **`GET /api/health`** (DB ping), catalog + baskets.
pub fn app(pool: Arc<PgPool>) -> Router {
    api::router(pool)
}
