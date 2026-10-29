use shop::{Basket, BasketLine, Product};

pub fn mug() -> (Basket, [Product; 1]) {
    let catalog = [Product {
        id: 1,
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
