//! Verify basket routes are registered under /api (regression: nest + path params).

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

#[tokio::test]
async fn post_then_get_basket_round_trip() {
    let app = common::router_with_migrated_db_and_product1().await;

    let basket_id = format!(
        "test-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let post_uri = format!("/api/baskets/{}/items", basket_id);
    let get_uri = format!("/api/baskets/{}", basket_id);

    let post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&post_uri)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":2}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(
        post.status(),
        StatusCode::NO_CONTENT,
        "POST basket item: {:?}",
        post.status()
    );

    let get = app
        .oneshot(
            Request::builder()
                .uri(&get_uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(get.status(), StatusCode::OK);
    let body = axum::body::to_bytes(get.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(v["subtotal_cents"], 200);
    assert_eq!(v["lines"].as_array().unwrap().len(), 1);
}
