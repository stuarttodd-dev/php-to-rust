//! OPTIONS preflight must not be 405 (browser CORS). No database.

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::routing::{get, on, MethodFilter};
use axum::Router;
use tower::ServiceExt;
use tower_http::cors::CorsLayer;

async fn cors_preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[tokio::test]
async fn options_preflight_on_products_is_not_405() {
    let app = Router::new()
        .route("/api/products", get(|| async { "ok" }))
        .route(
            "/api/products",
            on(MethodFilter::OPTIONS, cors_preflight),
        )
        .layer(CorsLayer::permissive());

    let res = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/products")
                .header("Origin", "http://127.0.0.1:8080")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
}

/// Same order as production: `.layer(CorsLayer).with_state(())`.
#[tokio::test]
async fn options_preflight_with_state_order() {
    let app = Router::new()
        .route("/api/products", get(|| async { "ok" }))
        .route(
            "/api/products",
            on(MethodFilter::OPTIONS, cors_preflight),
        )
        .layer(CorsLayer::permissive())
        .with_state(());

    let res = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/products")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_ne!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
}
