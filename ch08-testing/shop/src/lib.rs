/// Cents.
///
/// ```
/// # use shop::Money;
/// assert_eq!(Money(400).0 + Money(600).0, 1000);
/// ```
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

fn line_cents(price_cents: u32, quantity: u32) -> Result<u32, BasketError> {
    price_cents
        .checked_mul(quantity)
        .ok_or(BasketError::Overflow)
}

impl Basket {
    /// Totals the lines.
    ///
    /// ```
    /// # use shop::{Basket, BasketLine, Money, Product};
    /// let catalog = [Product {
    ///     id: 1,
    ///     price_cents: 1000,
    /// }];
    /// let basket = Basket {
    ///     lines: vec![BasketLine {
    ///         product_id: 1,
    ///         quantity: 1,
    ///     }],
    /// };
    /// assert_eq!(basket.total(&catalog).unwrap(), Money(1000));
    /// ```
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn totals_the_mug() {
        assert_eq!(line_cents(1000, 1).unwrap(), 1000);
        let catalog = [Product {
            id: 1,
            price_cents: 1000,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 1,
            }],
        };
        assert_eq!(basket.total(&catalog).unwrap(), Money(1000), "one mug");
    }
}
