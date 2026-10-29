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

pub struct MultiBuy {
    pub product_id: u64,
    pub buy: u32,
    pub pay_for: u32,
}

impl PriceRule for MultiBuy {
    fn apply(
        &self,
        running: Money,
        basket: &Basket,
        catalog: &[Product],
    ) -> Result<Money, BasketError> {
        let mut unit_price = 0u32;
        for product in catalog {
            if product.id == self.product_id {
                unit_price = product.price_cents;
                break;
            }
        }
        let mut quantity = 0u32;
        for line in &basket.lines {
            if line.product_id == self.product_id {
                quantity = quantity
                    .checked_add(line.quantity)
                    .ok_or(BasketError::Overflow)?;
            }
        }
        let Some(groups) = quantity.checked_div(self.buy) else {
            return Ok(running);
        };
        let free_per_group = self
            .buy
            .checked_sub(self.pay_for)
            .ok_or(BasketError::Overflow)?;
        let free_units = groups
            .checked_mul(free_per_group)
            .ok_or(BasketError::Overflow)?;
        let off = unit_price
            .checked_mul(free_units)
            .ok_or(BasketError::Overflow)?;
        Ok(Money(running.0.saturating_sub(off)))
    }
}

pub fn apply_one<R: PriceRule>(
    rule: &R,
    running: Money,
    basket: &Basket,
    catalog: &[Product],
) -> Result<Money, BasketError> {
    rule.apply(running, basket, catalog)
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

#[cfg(test)]
mod tests {
    use super::*;
    use shop::BasketLine;

    fn mug() -> (Basket, [Product; 1]) {
        let catalog = [Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 1,
            }],
        };
        (basket, catalog)
    }

    #[test]
    fn percent_then_fixed() {
        let (basket, catalog) = mug();
        let rules: Vec<Box<dyn PriceRule>> =
            vec![Box::new(PercentOff(10)), Box::new(FixedOff(Money(100)))];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            Money(800)
        );
    }

    #[test]
    fn fixed_then_percent() {
        let (basket, catalog) = mug();
        let rules: Vec<Box<dyn PriceRule>> =
            vec![Box::new(FixedOff(Money(100))), Box::new(PercentOff(10))];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            Money(810)
        );
    }

    #[test]
    fn fixed_bigger_than_the_total_is_zero() {
        let (basket, catalog) = mug();
        let rules: Vec<Box<dyn PriceRule>> = vec![Box::new(FixedOff(Money(5_000)))];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            Money(0)
        );
    }

    #[test]
    fn percent_overflow_is_an_error() {
        let catalog = [Product {
            id: 1,
            title: String::from("mug"),
            price_cents: u32::MAX,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 1,
            }],
        };
        let rules: Vec<Box<dyn PriceRule>> = vec![Box::new(PercentOff(2))];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap_err(),
            BasketError::Overflow
        );
    }

    #[test]
    fn empty_rules_match_the_basket_total() {
        let (basket, catalog) = mug();
        let rules: Vec<Box<dyn PriceRule>> = Vec::new();
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            basket.total(&catalog).unwrap()
        );
    }

    #[test]
    fn three_mugs_pay_for_two() {
        let catalog = [Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 3,
            }],
        };
        let rules: Vec<Box<dyn PriceRule>> = vec![Box::new(MultiBuy {
            product_id: 1,
            buy: 3,
            pay_for: 2,
        })];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            Money(2000)
        );
    }

    #[test]
    fn two_mugs_get_no_multibuy() {
        let catalog = [Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        }];
        let basket = Basket {
            lines: vec![BasketLine {
                product_id: 1,
                quantity: 2,
            }],
        };
        let rules: Vec<Box<dyn PriceRule>> = vec![Box::new(MultiBuy {
            product_id: 1,
            buy: 3,
            pay_for: 2,
        })];
        assert_eq!(
            price_with_rules(&basket, &catalog, &rules).unwrap(),
            Money(2000)
        );
    }

    #[test]
    fn apply_one_uses_the_concrete_rule() {
        let (basket, catalog) = mug();
        let running = basket.total(&catalog).unwrap();
        let rule = PercentOff(10);
        assert_eq!(
            apply_one(&rule, running, &basket, &catalog).unwrap(),
            Money(900)
        );
        assert_eq!(rule.apply(running, &basket, &catalog).unwrap(), Money(900));
    }
}
