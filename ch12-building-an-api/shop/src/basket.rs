use std::fmt;

use crate::money::Money;
use crate::product::Product;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasketLine {
    pub product_id: u64,
    pub quantity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basket {
    pub lines: Vec<BasketLine>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Order {
    Open(Basket),
    CheckedOut { total: Money },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasketError {
    ZeroQuantity,
    Overflow,
}

impl fmt::Display for BasketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroQuantity => write!(f, "quantity must be greater than zero"),
            Self::Overflow => write!(f, "line total overflowed"),
        }
    }
}

impl std::error::Error for BasketError {}

impl Basket {
    pub fn add(&mut self, product_id: u64, qty: u32) -> Result<(), BasketError> {
        if qty == 0 {
            return Err(BasketError::ZeroQuantity);
        }
        let mut found: Option<usize> = None;
        for i in 0..self.lines.len() {
            if self.lines[i].product_id == product_id {
                found = Some(i);
                break;
            }
        }
        match found {
            Some(i) => {
                self.lines[i].quantity = self.lines[i]
                    .quantity
                    .checked_add(qty)
                    .ok_or(BasketError::Overflow)?;
            }
            None => {
                self.lines.push(BasketLine {
                    product_id,
                    quantity: qty,
                });
            }
        }
        Ok(())
    }

    pub fn remove(&mut self, product_id: u64) {
        let mut found: Option<usize> = None;
        for i in 0..self.lines.len() {
            if self.lines[i].product_id == product_id {
                found = Some(i);
                break;
            }
        }
        if let Some(i) = found {
            self.lines.remove(i);
        }
    }

    pub fn total(&self, catalog: &[Product]) -> Result<Money, BasketError> {
        let mut cents: u32 = 0;
        for line in &self.lines {
            for product in catalog {
                if product.id == line.product_id {
                    let line_cents = product
                        .price_cents
                        .checked_mul(line.quantity)
                        .ok_or(BasketError::Overflow)?;
                    cents = cents.checked_add(line_cents).ok_or(BasketError::Overflow)?;
                }
            }
        }
        Ok(Money(cents))
    }

    pub fn checkout(self, catalog: &[Product]) -> Result<Order, BasketError> {
        let total = self.total(catalog)?;
        Ok(Order::CheckedOut { total })
    }
}

#[cfg(test)]
mod tests {
    use super::{Basket, BasketError, Order};
    use crate::money::Money;
    use crate::product::Product;

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
    fn merges_quantity_for_the_same_product() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 1).unwrap();
        basket.add(1, 2).unwrap();
        assert_eq!(basket.lines.len(), 1);
        assert_eq!(basket.lines[0].product_id, 1);
        assert_eq!(basket.lines[0].quantity, 3);
    }

    #[test]
    fn adds_a_second_product() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 1).unwrap();
        basket.add(2, 1).unwrap();
        assert_eq!(basket.lines.len(), 2);
        assert_eq!(basket.lines[1].product_id, 2);
        assert_eq!(basket.lines[1].quantity, 1);
    }

    #[test]
    fn removes_a_product() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 1).unwrap();
        basket.add(2, 1).unwrap();
        basket.remove(1);
        assert_eq!(basket.lines.len(), 1);
        assert_eq!(basket.lines[0].product_id, 2);
        basket.remove(9);
        assert_eq!(basket.lines.len(), 1);
    }

    #[test]
    fn totals_price_times_quantity() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 2).unwrap();
        basket.add(2, 1).unwrap();
        assert_eq!(basket.total(&catalog()).unwrap(), Money(1000 * 2 + 399));
        assert_eq!(
            Basket { lines: Vec::new() }.total(&catalog()).unwrap(),
            Money(0)
        );
    }

    #[test]
    fn checkout_returns_the_total() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 2).unwrap();
        let order = basket.checkout(&catalog()).unwrap();
        let total = match order {
            Order::CheckedOut { total } => total,
            Order::Open(_) => Money(0),
        };
        assert_eq!(total, Money(2000));
    }

    #[test]
    fn rejects_zero_quantity() {
        let mut basket = Basket { lines: Vec::new() };
        let error = basket.add(1, 0).unwrap_err();
        assert_eq!(error, BasketError::ZeroQuantity);
        assert!(basket.lines.is_empty());
    }

    #[test]
    fn merged_quantity_overflow_is_an_error() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, u32::MAX).unwrap();
        assert_eq!(basket.add(1, 1).unwrap_err(), BasketError::Overflow);
        assert_eq!(basket.lines[0].quantity, u32::MAX);
    }

    #[test]
    fn overflow_is_an_error() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 2).unwrap();
        let catalog = [Product {
            id: 1,
            title: String::from("mug"),
            price_cents: u32::MAX,
        }];
        assert_eq!(basket.total(&catalog).unwrap_err(), BasketError::Overflow);
        assert_eq!(
            basket.checkout(&catalog).unwrap_err(),
            BasketError::Overflow
        );
    }
}
