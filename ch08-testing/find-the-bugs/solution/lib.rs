#[derive(Debug, PartialEq, Eq)]
pub struct Money(pub u32);

#[derive(Debug, PartialEq, Eq)]
pub enum BasketError {
    Overflow,
}

pub struct Product {
    pub id: u64,
    pub price_cents: u32,
}

pub struct BasketLine {
    pub product_id: u64,
    pub quantity: u32,
}

pub struct Basket {
    pub lines: Vec<BasketLine>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Order {
    CheckedOut { total: Money },
}

pub struct PercentOff(pub u32);

fn line_cents(price_cents: u32, quantity: u32) -> Result<u32, BasketError> {
    price_cents
        .checked_mul(quantity)
        .ok_or(BasketError::Overflow)
}

impl Basket {
    pub fn total(&self, catalog: &[Product]) -> Result<Money, BasketError> {
        let mut cents = 0u32;
        for line in &self.lines {
            for product in catalog {
                if product.id == line.product_id {
                    let line_total = line_cents(product.price_cents, line.quantity)?;
                    cents = cents.checked_add(line_total).ok_or(BasketError::Overflow)?;
                }
            }
        }
        Ok(Money(cents))
    }

    pub fn checkout(&self, catalog: &[Product]) -> Result<Order, BasketError> {
        let total = self.total(catalog)?;
        Ok(Order::CheckedOut { total })
    }
}

impl PercentOff {
    pub fn apply(&self, price: Money) -> Result<Money, BasketError> {
        let keep = 100u32.checked_sub(self.0).ok_or(BasketError::Overflow)?;
        let cents = price.0.checked_mul(keep).ok_or(BasketError::Overflow)? / 100;
        Ok(Money(cents))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_mugs() -> (Basket, [Product; 1]) {
        let catalog = [Product {
            id: 1,
            price_cents: 1000,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 2,
            }],
        };
        (basket, catalog)
    }

    #[test]
    fn two_mugs_are_2000() {
        let (basket, catalog) = two_mugs();
        assert_eq!(basket.total(&catalog).unwrap(), Money(2000));
    }

    #[test]
    fn ten_percent_of_two_mugs_is_1800() {
        assert_eq!(PercentOff(10).apply(Money(2000)).unwrap(), Money(1800));
    }

    #[test]
    fn checkout_returns_the_basket_total() {
        let (basket, catalog) = two_mugs();
        assert_eq!(
            basket.checkout(&catalog).unwrap(),
            Order::CheckedOut { total: Money(2000) }
        );
    }
}
