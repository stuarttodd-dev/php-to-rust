use shop::{Basket, BasketError, Product};
use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrderLineError {
    #[error("order line is malformed")]
    Malformed,
    #[error("bad quantity: {0}")]
    BadQuantity(#[from] ParseIntError),
    #[error("unknown product {0}")]
    UnknownProduct(u64),
    #[error(transparent)]
    Basket(#[from] BasketError),
}

pub fn parse_line(raw: &str, catalog: &[Product]) -> Result<(u64, u32), OrderLineError> {
    let _ = (raw, catalog);
    todo!("should return the product id and quantity");
}

pub fn load_order(text: &str, catalog: &[Product]) -> Result<Basket, OrderLineError> {
    let _ = (text, catalog);
    todo!("should build a basket from the order text");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn catalog() -> [Product; 2] {
        [
            Product {
                id: 1,
                title: String::from("mug"),
                price_cents: 1000,
            },
            Product {
                id: 2,
                title: String::from("tea"),
                price_cents: 399,
            },
        ]
    }

    #[test]
    fn loads_lines_and_skips_blanks() {
        let basket = load_order("1,2\n\n2,1\n", &catalog()).unwrap();
        assert_eq!(basket.lines.len(), 2);
        assert_eq!(basket.lines[0].quantity, 2);
        assert_eq!(basket.lines[1].product_id, 2);
    }

    #[test]
    fn merges_a_repeated_product() {
        let basket = load_order("1,2\n1,1\n", &catalog()).unwrap();
        assert_eq!(basket.lines.len(), 1);
        assert_eq!(basket.lines[0].quantity, 3);
    }

    #[test]
    fn rejects_a_line_with_no_comma() {
        let error = load_order("1\n", &catalog()).unwrap_err();
        assert!(matches!(error, OrderLineError::Malformed));
    }

    #[test]
    fn rejects_a_non_numeric_product_id() {
        let error = load_order("nope,1\n", &catalog()).unwrap_err();
        assert!(matches!(error, OrderLineError::Malformed));
    }

    #[test]
    fn rejects_a_bad_quantity() {
        let error = load_order("1,nope\n", &catalog()).unwrap_err();
        assert!(matches!(error, OrderLineError::BadQuantity(_)));
    }

    #[test]
    fn rejects_an_unknown_product() {
        let error = load_order("9,1\n", &catalog()).unwrap_err();
        assert!(matches!(error, OrderLineError::UnknownProduct(9)));
    }

    #[test]
    fn rejects_zero_quantity() {
        let error = load_order("1,0\n", &catalog()).unwrap_err();
        assert!(matches!(
            error,
            OrderLineError::Basket(BasketError::ZeroQuantity)
        ));
    }

    #[test]
    fn rejects_a_merged_quantity_overflow() {
        let text = format!("1,{}\n1,1\n", u32::MAX);
        let error = load_order(&text, &catalog()).unwrap_err();
        assert!(matches!(
            error,
            OrderLineError::Basket(BasketError::Overflow)
        ));
    }
}
