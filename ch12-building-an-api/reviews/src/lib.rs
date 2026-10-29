use axum::Json;
use axum::Router;
use axum::extract::Path;
use axum::routing::get;
use serde::{Deserialize, Serialize};

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

pub fn app() -> Router {
    Router::new()
        .route("/products/{product_id}/reviews", get(list_reviews).post(create_review))
}

async fn list_reviews(Path(product_id): Path<i64>) -> Json<Vec<Review>> {
    let _ = product_id;
    Json(vec![])
}

async fn create_review(
    Path(product_id): Path<i64>,
    Json(body): Json<NewReview>,
) -> Result<Json<Review>, &'static str> {
    body.validate()?;
    Ok(Json(Review {
        id: 1,
        product_id,
        rating: body.rating,
        body: body.body,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn get_reviews_returns_json_array() {
        let response = app()
            .oneshot(
                axum::http::Request::builder()
                    .uri("/products/1/reviews")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert!(response.status().is_success());
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"[]");
    }

    #[test]
    fn rejects_bad_rating() {
        let review = NewReview {
            rating: 6,
            body: String::from("too hot"),
        };
        assert_eq!(review.validate(), Err("rating must be between 1 and 5"));
    }
}
