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

impl Basket {
    pub fn add(&mut self, product_id: u64, quantity: u32) {
        let mut found: Option<usize> = None;
        for i in 0..self.lines.len() {
            if self.lines[i].product_id == product_id {
                found = Some(i);
                break;
            }
        }
        match found {
            Some(i) => {
                self.lines[i].quantity += quantity;
            }
            None => {
                self.lines.push(BasketLine {
                    product_id,
                    quantity,
                });
            }
        }
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

    pub fn total(&self, catalog: &[Product]) -> Money {
        let mut cents: u32 = 0;
        for line in &self.lines {
            for product in catalog {
                if product.id == line.product_id {
                    let line_cents = match product.price_cents.checked_mul(line.quantity) {
                        Some(value) => value,
                        None => return Money(u32::MAX),
                    };
                    cents = match cents.checked_add(line_cents) {
                        Some(value) => value,
                        None => return Money(u32::MAX),
                    };
                }
            }
        }
        Money(cents)
    }

    pub fn checkout(self, catalog: &[Product]) -> Order {
        let total = self.total(catalog);
        Order::CheckedOut { total }
    }
}

#[cfg(test)]
mod tests {
    use super::{Basket, Order};
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
        basket.add(1, 1);
        basket.add(1, 2);
        assert_eq!(basket.lines.len(), 1);
        assert_eq!(basket.lines[0].product_id, 1);
        assert_eq!(basket.lines[0].quantity, 3);
    }

    #[test]
    fn adds_a_second_product() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 1);
        basket.add(2, 1);
        assert_eq!(basket.lines.len(), 2);
        assert_eq!(basket.lines[1].product_id, 2);
        assert_eq!(basket.lines[1].quantity, 1);
    }

    #[test]
    fn removes_a_product() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 1);
        basket.add(2, 1);
        basket.remove(1);
        assert_eq!(basket.lines.len(), 1);
        assert_eq!(basket.lines[0].product_id, 2);
        basket.remove(9);
        assert_eq!(basket.lines.len(), 1);
    }

    #[test]
    fn totals_price_times_quantity() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 2);
        basket.add(2, 1);
        assert_eq!(basket.total(&catalog()), Money(1000 * 2 + 399));
        assert_eq!(Basket { lines: Vec::new() }.total(&catalog()), Money(0));
    }

    #[test]
    fn checkout_returns_the_total() {
        let mut basket = Basket { lines: Vec::new() };
        basket.add(1, 2);
        let order = basket.checkout(&catalog());
        let total = match order {
            Order::CheckedOut { total } => total,
            Order::Open(_) => Money(0),
        };
        assert_eq!(total, Money(2000));
    }
}
