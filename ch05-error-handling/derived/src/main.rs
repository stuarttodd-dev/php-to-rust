use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
enum OrderLineError {
    #[error("order line is empty")]
    Empty,
    #[error("order line is malformed")]
    Malformed,
    #[error("bad quantity: {0}")]
    BadQuantity(#[from] ParseIntError),
    #[error("unknown product {0}")]
    UnknownProduct(u64),
}

fn parse_line(raw: &str, catalog: &[u64]) -> Result<(u64, u32), OrderLineError> {
    if raw.is_empty() {
        return Err(OrderLineError::Empty);
    }
    let (id_text, qty_text) = match raw.split_once(',') {
        Some(parts) => parts,
        None => return Err(OrderLineError::Malformed),
    };
    let product_id: u64 = match id_text.parse() {
        Ok(id) => id,
        Err(_) => return Err(OrderLineError::Malformed),
    };
    let quantity: u32 = qty_text.parse()?;
    if !catalog.contains(&product_id) {
        return Err(OrderLineError::UnknownProduct(product_id));
    }
    Ok((product_id, quantity))
}

fn main() {
    let catalog = [1u64];
    for raw in ["1,2", "nope,1", "1,nope"] {
        match parse_line(raw, &catalog) {
            Ok((id, quantity)) => println!("{id} {quantity}"),
            Err(error) => println!("{error}"),
        }
    }
}
