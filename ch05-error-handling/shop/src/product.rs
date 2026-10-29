#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub price_cents: u32,
}
