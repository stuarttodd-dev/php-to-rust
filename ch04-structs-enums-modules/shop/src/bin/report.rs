fn main() {
    let cents = shop::Product {
        id: 1,
        title: String::from("mug"),
        price_cents: 1000,
    }
    .price_cents;
    println!("report {cents}");
}
