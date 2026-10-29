use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn mug_total(code: &str, qty: u32) -> Option<u32> {
    let _ = (code, qty);
    todo!()
}

pub async fn add_mug(basket_id: i32) -> Result<(), String> {
    let _ = basket_id;
    todo!()
}
