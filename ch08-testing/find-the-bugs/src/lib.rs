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
                    let line_total = line_cents(product.price_cents, 1)?;
                    cents = cents.checked_add(line_total).ok_or(BasketError::Overflow)?;
                }
            }
        }
        Ok(Money(cents))
    }

    pub fn checkout(&self, catalog: &[Product]) -> Result<Order, BasketError> {
        let _ = (self, catalog);
        Ok(Order::CheckedOut { total: Money(0) })
    }
}

impl PercentOff {
    pub fn apply(&self, price: Money) -> Result<Money, BasketError> {
        let cents = price.0.checked_sub(self.0).ok_or(BasketError::Overflow)?;
        Ok(Money(cents))
    }
}
