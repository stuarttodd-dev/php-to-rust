mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::connect_test_pool;
use shop_api::app;
use std::sync::Arc;
use tower::ServiceExt;

#[tokio::test]
async fn get_api_health_returns_200() {
    let pool = connect_test_pool().await;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("run migrations");

    let pool = Arc::new(pool);
    let app = app(pool);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oneshot");

    assert_eq!(response.status(), StatusCode::OK);
}
