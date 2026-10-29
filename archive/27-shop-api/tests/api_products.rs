mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::connect_test_pool;
use shop_api::app;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn get_api_products_returns_200_json() {
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
    .expect("seed products for list test");

    let pool = Arc::new(pool);
    let app = app(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");

    assert_eq!(response.status(), StatusCode::OK);
    let ctype = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok());
    assert!(
        ctype.is_some_and(|c| c.starts_with("application/json")),
        "expected application/json, got {:?}",
        ctype
    );

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let s = String::from_utf8_lossy(&body);
    assert!(
        !s.contains("cost_cents"),
        "list response must not expose internal fields: {s}"
    );
}
