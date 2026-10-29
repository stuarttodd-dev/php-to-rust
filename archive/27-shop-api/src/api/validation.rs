//! Pure validation for basket item payloads (testable without HTTP).

use super::dto::{AddBasketItemBody, PatchBasketItemBody};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasketItemValidationError {
    QtyMustBePositive,
}

pub fn validate_add_item(body: &AddBasketItemBody) -> Result<(), BasketItemValidationError> {
    if body.qty == 0 {
        return Err(BasketItemValidationError::QtyMustBePositive);
    }
    Ok(())
}

pub fn validate_patch_qty(body: &PatchBasketItemBody) -> Result<(), BasketItemValidationError> {
    if body.qty == 0 {
        return Err(BasketItemValidationError::QtyMustBePositive);
    }
    Ok(())
}

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn add_rejects_zero_qty() {
        let body = AddBasketItemBody {
            product_id: 1,
            qty: 0,
        };
        assert_eq!(
            validate_add_item(&body).unwrap_err(),
            BasketItemValidationError::QtyMustBePositive
        );
    }

    #[test]
    fn add_accepts_positive_qty() {
        let body = AddBasketItemBody {
            product_id: 42,
            qty: 3,
        };
        assert!(validate_add_item(&body).is_ok());
    }

    #[test]
    fn patch_rejects_zero_qty() {
        let body = PatchBasketItemBody { qty: 0 };
        assert_eq!(
            validate_patch_qty(&body).unwrap_err(),
            BasketItemValidationError::QtyMustBePositive
        );
    }
}
