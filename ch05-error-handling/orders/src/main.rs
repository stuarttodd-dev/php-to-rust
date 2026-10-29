use anyhow::Context;
use shop::Product;

fn main() -> anyhow::Result<()> {
    let text = std::fs::read_to_string("orders.txt").context("loading order")?;
    let catalog = [
        Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        },
        Product {
            id: 2,
            title: String::from("tea"),
            price_cents: 399,
        },
    ];
    let basket = orders::load_order(&text, &catalog)?;
    let count = basket.lines.len();
    println!("{count}");
    Ok(())
}
