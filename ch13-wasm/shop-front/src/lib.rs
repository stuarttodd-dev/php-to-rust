use discounts::{PriceRule, price_with_rules, rule_for};
use gloo_net::http::Request;
use serde::Serialize;
use shop::{Basket, Product};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct NewLine {
    product_id: i32,
    qty: u32,
}

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

pub async fn add_mug(basket_id: i32) -> Result<(), String> {
    Request::post(&format!("http://127.0.0.1:8090/baskets/{basket_id}/lines"))
        .json(&NewLine {
            product_id: 1,
            qty: 1,
        })
        .map_err(|err| err.to_string())?
        .send()
        .await
        .map_err(|err| err.to_string())?;
    Ok(())
}
