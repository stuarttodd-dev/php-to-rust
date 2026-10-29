//! HTTP response DTOs (Laravel-style API resources): stable wire shape, no internal fields.

use crate::domain::Product;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProductResponse {
    pub id: u64,
    pub title: String,
    pub price_cents: u32,
}

impl From<&Product> for ProductResponse {
    fn from(p: &Product) -> Self {
        Self {
            id: p.id,
            title: p.title.clone(),
            price_cents: p.price_cents,
        }
    }
}

#[cfg(test)]
mod product_response_tests {
    use super::ProductResponse;
    use crate::domain::Product;

    #[test]
    fn json_does_not_include_cost_cents() {
        let p = Product {
            id: 1,
            title: "Rust mug".into(),
            price_cents: 1299,
            cost_cents: 400,
        };
        let r = ProductResponse::from(&p);
        let v = serde_json::to_value(&r).expect("serialize");
        assert!(v.get("cost_cents").is_none(), "cost must not leak: {v:?}");
        assert_eq!(v["id"], 1);
        assert_eq!(v["title"], "Rust mug");
        assert_eq!(v["price_cents"], 1299);
    }
}
