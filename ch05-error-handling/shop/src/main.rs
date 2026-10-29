use shop::{Basket, BasketError, Money, Order, Product};

fn describe(order: &Order) -> String {
    match order {
        Order::Open(inner) => inner.lines.len().to_string(),
        Order::CheckedOut { total } => total.0.to_string(),
    }
}

fn main() -> Result<(), BasketError> {
    let catalog = [Product {
        id: 1,
        title: String::from("mug"),
        price_cents: 1000,
    }];
    let mut basket = Basket { lines: Vec::new() };
    basket.add(1, 2)?;
    let total = basket.total(&catalog)?;
    let cents = total.0;
    println!("{cents}");
    basket.remove(1);
    let order = basket.checkout(&catalog)?;
    let open = Order::Open(Basket { lines: Vec::new() });
    let checked = Order::CheckedOut { total: Money(0) };
    println!("{}", describe(&open));
    println!("{}", describe(&checked));
    println!("{}", describe(&order));
    Ok(())
}
