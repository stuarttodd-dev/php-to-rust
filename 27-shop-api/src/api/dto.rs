//! JSON bodies for basket HTTP endpoints (request DTOs).

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddBasketItemBody {
    pub product_id: u64,
    pub qty: u32,
}

#[derive(Debug, Deserialize)]
pub struct PatchBasketItemBody {
    pub qty: u32,
}
