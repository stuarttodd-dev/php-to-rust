mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::connect_test_pool;
use shop_api::app;
use std::sync::Arc;
use tower::ServiceExt;

async fn pool_with_migrations_and_seed() -> sqlx::PgPool {
    let pool = connect_test_pool().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("run migrations");

    sqlx::query(
        r#"
        INSERT INTO products (id, title, price_cents) VALUES
          (1, 'Rust mug', 1299),
          (2, 'Ferris sticker', 399)
        ON CONFLICT (id) DO UPDATE SET
          title = EXCLUDED.title,
          price_cents = EXCLUDED.price_cents
        "#,
    )
    .execute(&pool)
    .await
    .expect("seed products");

    pool
}

#[tokio::test]
async fn get_product_returns_200_when_present() {
    let pool = pool_with_migrations_and_seed().await;
    let app = app(Arc::new(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn get_product_hides_cost_cents() {
    let pool = pool_with_migrations_and_seed().await;

    sqlx::query("UPDATE products SET cost_cents = 777 WHERE id = 1")
        .execute(&pool)
        .await
        .expect("set internal cost");

    let app = app(Arc::new(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let s = String::from_utf8_lossy(&body);
    assert!(
        !s.contains("cost_cents"),
        "response must not expose internal fields: {s}"
    );
    assert!(
        !s.contains("777"),
        "internal cost value must not appear in JSON: {s}"
    );
}

#[tokio::test]
async fn get_product_returns_404_when_missing() {
    let pool = pool_with_migrations_and_seed().await;
    let app = app(Arc::new(pool));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products/999")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
