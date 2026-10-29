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
        basket: &Basket,
        catalog: &[Product],
    ) -> Result<Money, BasketError> {
        let _ = (self, running, basket, catalog);
        todo!("should apply the percent to the running total");
    }
}

pub struct FixedOff(pub Money);

impl PriceRule for FixedOff {
    fn apply(
        &self,
        running: Money,
        basket: &Basket,
        catalog: &[Product],
    ) -> Result<Money, BasketError> {
        let _ = (self, running, basket, catalog);
        todo!("should subtract the fixed amount from the running total");
    }
}

pub fn apply_one<R: PriceRule>(
    rule: &R,
    running: Money,
    basket: &Basket,
    catalog: &[Product],
) -> Result<Money, BasketError> {
    let _ = (rule, running, basket, catalog);
    todo!("should apply one rule");
}

pub fn price_with_rules(
    basket: &Basket,
    catalog: &[Product],
    rules: &[Box<dyn PriceRule>],
) -> Result<Money, BasketError> {
    let _ = (basket, catalog, rules);
    todo!("should price the basket after every rule");
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
