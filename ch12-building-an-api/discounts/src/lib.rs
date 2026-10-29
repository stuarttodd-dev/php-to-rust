use shop::{Basket, BasketError, Money, Product};

pub trait PriceRule {
    fn apply(
        &self,
        running: Money,
        basket: &Basket,
        catalog: &[Product],
    ) -> Result<Money, BasketError>;
}

pub struct PercentOff(pub u32);

impl PriceRule for PercentOff {
    fn apply(
        &self,
        running: Money,
        _basket: &Basket,
        _catalog: &[Product],
    ) -> Result<Money, BasketError> {
        let keep = 100u32.checked_sub(self.0).ok_or(BasketError::Overflow)?;
        let cents = running.0.checked_mul(keep).ok_or(BasketError::Overflow)? / 100;
        Ok(Money(cents))
    }
}

pub struct FixedOff(pub Money);

impl PriceRule for FixedOff {
    fn apply(
        &self,
        running: Money,
        _basket: &Basket,
        _catalog: &[Product],
    ) -> Result<Money, BasketError> {
        Ok(Money(running.0.saturating_sub(self.0.0)))
    }
}

pub fn price_with_rules(
    basket: &Basket,
    catalog: &[Product],
    rules: &[Box<dyn PriceRule>],
) -> Result<Money, BasketError> {
    let mut running = basket.total(catalog)?;
    for rule in rules {
        running = rule.apply(running, basket, catalog)?;
    }
    Ok(running)
}

pub fn rule_for(code: &str) -> Option<Box<dyn PriceRule>> {
    match code {
        "SAVE10" => Some(Box::new(PercentOff(10))),
        "TAKE100" => Some(Box::new(FixedOff(Money(100)))),
        _ => None,
    }
}
