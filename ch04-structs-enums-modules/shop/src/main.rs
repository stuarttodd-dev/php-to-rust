use shop::{Basket, Money, Order, Product};

fn describe(order: &Order) -> String {
    match order {
        Order::Open(inner) => format!("{}", inner.lines.len()),
        Order::CheckedOut { total } => format!("{}", total.0),
    }
}

fn main() {
    let catalog = [Product {
        id: 1,
        title: String::from("mug"),
        price_cents: 1000,
    }];
    let shown = &catalog[0];
    println!("{} {} {}", shown.id, shown.title, shown.price_cents);

    let mut basket = Basket { lines: Vec::new() };
    basket.add(1, 2);
    let cents = basket.total(&catalog);
    println!("{}", cents.0);
    basket.remove(1);
    let order = basket.checkout(&catalog);
    let open = Order::Open(Basket { lines: Vec::new() });
    let checked = Order::CheckedOut { total: Money(0) };
    println!("{}", describe(&open));
    println!("{}", describe(&checked));
    println!("{}", describe(&order));
}
