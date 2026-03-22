//! Shared helpers for integration tests (Postgres required).

use axum::Router;
use shop_api::app;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;

/// Migrations + product `id = 1` for basket / validation tests.
#[allow(dead_code)] // Only imported from basket `tests/*.rs`; other integration crates still `mod common`.
pub async fn router_with_migrated_db_and_product1() -> Router {
    let pool = connect_test_pool().await;
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("migrations");
    sqlx::query(
        r#"
        INSERT INTO products (id, title, price_cents) VALUES (1, 'Test', 100)
        ON CONFLICT (id) DO UPDATE SET title = EXCLUDED.title, price_cents = EXCLUDED.price_cents
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed product 1");
    app(Arc::new(pool))
}

pub async fn connect_test_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        panic!(
            "DATABASE_URL is not set. Copy .env.example to .env or export it.\n\
             For tests on your machine (outside Docker), use host 127.0.0.1, not db:\n\
             DATABASE_URL=postgres://user:secret@127.0.0.1:5432/mydb\n\
             Start Postgres: docker compose up -d db"
        )
    });
    PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .connect(&url)
        .await
        .unwrap_or_else(|e| {
            panic!(
                "Postgres connection failed ({e}).\n\
                 - Start the DB: cd 27-shop-api && docker compose up -d db\n\
                 - Use host 127.0.0.1 in DATABASE_URL for host-side tests (not hostname `db`).\n\
                 - Ensure port 5432 is free / matches compose.yml."
            )
        })
}
