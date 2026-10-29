fn total_cents(prices: &[u32]) -> u32 {
    prices.iter().copied().sum()
}

fn affordable_names<'a>(items: &[(&'a str, u32)], cap_cents: u32) -> Vec<&'a str> {
    items
        .iter()
        .filter(|(_, cents)| *cents < cap_cents)
        .map(|(name, _)| *name)
        .collect()
}

fn first_price_above(prices: &[u32], t: u32) -> Option<u32> {
    prices.iter().copied().find(|&p| p > t)
}

fn main() {
    let prices = [199u32, 42, 500, 120];
    println!("total_cents (expect 861): {}", total_cents(&prices));

    let stock = [("tea", 199u32), ("laptop", 500u32), ("pen", 50u32)];
    println!(
        "affordable under 200¢: {:?}",
        affordable_names(&stock, 200)
    );

    if let Some(p) = first_price_above(&prices, 100) {
        println!("first price above 100¢: {}", p);
    } else {
        println!("no price above 100¢");
    }
}
