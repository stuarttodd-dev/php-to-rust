use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn bytes(response: axum::response::Response) -> Vec<u8> {
    response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec()
}

#[sqlx::test(migrations = "./migrations")]
async fn product_one_is_the_mug(pool: sqlx::PgPool) {
    let response = shop_api::app(pool)
        .oneshot(
            Request::builder()
                .uri("/products/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        bytes(response).await,
        br#"{"id":1,"title":"mug","price_cents":1000}"#.to_vec()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn missing_product_is_404(pool: sqlx::PgPool) {
    let response = shop_api::app(pool)
        .oneshot(
            Request::builder()
                .uri("/products/9")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        bytes(response).await,
        br#"{"error":"product not found"}"#.to_vec()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn checks_out_two_mugs(pool: sqlx::PgPool) {
    let app = shop_api::app(pool);
    let created = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(created.status(), StatusCode::CREATED);
    assert_eq!(bytes(created).await, br#"{"id":1}"#.to_vec());
    let added = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/lines")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":2}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(added.status(), StatusCode::CREATED);
    let checked_out = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/checkout")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(checked_out.status(), StatusCode::OK);
    assert_eq!(
        bytes(checked_out).await,
        br#"{"total_cents":2000}"#.to_vec()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn save10_prices_one_mug_at_900(pool: sqlx::PgPool) {
    let app = shop_api::app(pool);
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/lines")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/discount")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"code":"SAVE10"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(bytes(response).await, br#"{"total_cents":900}"#.to_vec());
}

#[sqlx::test(migrations = "./migrations")]
async fn take100_prices_one_mug_at_900(pool: sqlx::PgPool) {
    let app = shop_api::app(pool);
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/lines")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"product_id":1,"qty":1}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/discount")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"code":"TAKE100"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(bytes(response).await, br#"{"total_cents":900}"#.to_vec());
}

#[sqlx::test(migrations = "./migrations")]
async fn unknown_code_is_422(pool: sqlx::PgPool) {
    let app = shop_api::app(pool);
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/1/discount")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"code":"NOPE"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        bytes(response).await,
        br#"{"error":"unknown code"}"#.to_vec()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn known_code_on_a_missing_basket_is_404(pool: sqlx::PgPool) {
    let response = shop_api::app(pool)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/baskets/9/discount")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"code":"SAVE10"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        bytes(response).await,
        br#"{"error":"basket not found"}"#.to_vec()
    );
}
