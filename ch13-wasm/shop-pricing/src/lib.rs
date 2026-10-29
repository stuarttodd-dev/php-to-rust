use discounts::{PriceRule, price_with_rules, rule_for};
use shop::{Basket, Product};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mug_total(code: &str, qty: u32) -> Option<u32> {
    let mug = Product {
        id: 1,
        title: "mug".to_string(),
        price_cents: 1000,
    };
    let mut basket = Basket { lines: vec![] };
    basket.add(1, qty).ok()?;
    let rules: Vec<Box<dyn PriceRule>> = if code.is_empty() {
        vec![]
    } else {
        vec![rule_for(code)?]
    };
    Some(price_with_rules(&basket, &[mug], &rules).ok()?.0)
}
