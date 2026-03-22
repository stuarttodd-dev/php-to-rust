//! Basket POST/PATCH validation contract (JSON parse vs rule errors).

mod common;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use common::router_with_migrated_db_and_product1;
use tower::ServiceExt;

fn unique_basket_uri(prefix: &str) -> String {
    let n = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("/api/baskets/{}-{}/items", prefix, n)
}

#[tokio::test]
async fn post_basket_item_malformed_json_is_4xx() {
    let app = router_with_migrated_db_and_product1().await;
    let uri = unique_basket_uri("malformed");

    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&uri)
                .header("content-type", "application/json")
                .body(Body::from("{"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Axum 0.8: invalid JSON *syntax* → 400; shape/type mismatch vs target → 422.
    assert!(
        matches!(
            res.status(),
            StatusCode::BAD_REQUEST | StatusCode::UNPROCESSABLE_ENTITY
        ),
        "unexpected status: {}",
        res.status()
    );
}

#[tokio::test]
async fn post_basket_item_zero_qty_is_400_with_qty_field() {
    let app = router_with_migrated_db_and_product1().await;
    let uri = unique_basket_uri("zeroqty");

    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&uri)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["field"], "qty");
    assert!(
        v["error"]
            .as_str()
            .unwrap()
            .to_lowercase()
            .contains("qty"),
        "error should mention qty: {:?}",
        v["error"]
    );
}

#[tokio::test]
async fn post_basket_item_unknown_product_is_404() {
    let app = router_with_migrated_db_and_product1().await;
    let uri = unique_basket_uri("unknownprod");

    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&uri)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":99999,"qty":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["error"], "product not found");
}

#[tokio::test]
async fn post_basket_item_happy_path_is_204_then_get_shows_line() {
    let app = router_with_migrated_db_and_product1().await;
    let basket_id = format!(
        "happy-{}",
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
    assert_eq!(post.status(), StatusCode::NO_CONTENT);

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
    let bytes = axum::body::to_bytes(get.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["subtotal_cents"], 200);
    assert_eq!(v["lines"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn patch_basket_line_zero_qty_is_400_with_qty_field() {
    let app = router_with_migrated_db_and_product1().await;
    let basket_id = format!(
        "patchzero-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/baskets/{}/items", basket_id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/api/baskets/{}/items/1",
                    basket_id
                ))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"qty":0}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["field"], "qty");
}

#[tokio::test]
async fn patch_basket_line_happy_path_is_204() {
    let app = router_with_migrated_db_and_product1().await;
    let basket_id = format!(
        "patchok-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!("/api/baskets/{}/items", basket_id))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PATCH")
                .uri(format!(
                    "/api/baskets/{}/items/1",
                    basket_id
                ))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"qty":3}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NO_CONTENT);

    let get = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/baskets/{}", basket_id))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let bytes = axum::body::to_bytes(get.into_body(), usize::MAX)
        .await
        .unwrap();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["lines"][0]["quantity"], 3);
}
