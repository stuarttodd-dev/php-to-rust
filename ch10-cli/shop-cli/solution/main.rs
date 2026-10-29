use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub price_cents: u32,
}

#[derive(Parser)]
#[command(name = "shop-cli", bin_name = "shop-cli", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read a CSV catalog.
    Import { path: String },
    /// List the stored products.
    List,
    /// Price one product.
    Price {
        id: u64,
        #[arg(long, default_value_t = 1)]
        qty: u32,
    },
}

pub fn parse_catalog(text: &str) -> Result<Vec<Product>, csv::Error> {
    let mut reader = csv::Reader::from_reader(text.as_bytes());
    let mut products = Vec::new();
    for record in reader.deserialize() {
        products.push(record?);
    }
    Ok(products)
}

pub fn line_total(products: &[Product], id: u64, qty: u32) -> Result<u32, String> {
    let product = products
        .iter()
        .find(|product| product.id == id)
        .ok_or_else(|| format!("unknown id {id}"))?;
    product
        .price_cents
        .checked_mul(qty)
        .ok_or_else(|| format!("line total for id {id} overflowed"))
}

fn store_path() -> &'static str {
    "shop.json"
}

fn load_store() -> anyhow::Result<Vec<Product>> {
    let text = std::fs::read_to_string(store_path()).context("reading shop.json")?;
    Ok(serde_json::from_str(&text)?)
}

fn save_store(products: &[Product]) -> anyhow::Result<()> {
    let text = serde_json::to_string(products)?;
    std::fs::write(store_path(), text).context("writing shop.json")?;
    Ok(())
}

fn run() -> anyhow::Result<()> {
    match Cli::parse().command {
        Commands::Import { path } => {
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("reading {path}"))?;
            let products = parse_catalog(&text).with_context(|| format!("reading {path}"))?;
            save_store(&products)?;
            println!("read {}", products.len());
        }
        Commands::List => {
            for product in load_store()? {
                println!("{} {} {}", product.id, product.title, product.price_cents);
            }
        }
        Commands::Price { id, qty } => {
            let products = load_store()?;
            let total = line_total(&products, id, qty).map_err(anyhow::Error::msg)?;
            println!("{total}");
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    run()
}

#[cfg(test)]
mod tests {
    use super::{Product, line_total, parse_catalog};

    #[test]
    fn parses_two_products() {
        let text = "id,title,price_cents\n1,mug,1000\n2,bowl,2500\n";
        let products = parse_catalog(text).unwrap();
        assert_eq!(products.len(), 2);
        assert_eq!(products[0].title, "mug");
        assert_eq!(products[1].price_cents, 2500);
    }

    #[test]
    fn bad_row_names_the_line() {
        let text = "id,title,price_cents\n1,mug,abc\n";
        let message = parse_catalog(text).unwrap_err().to_string();
        assert!(message.contains("line: 2"), "{message}");
    }

    #[test]
    fn prices_two_mugs() {
        let products = vec![Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        }];
        assert_eq!(line_total(&products, 1, 2).unwrap(), 2000);
    }

    #[test]
    fn unknown_id_names_the_id() {
        let products = vec![Product {
            id: 1,
            title: String::from("mug"),
            price_cents: 1000,
        }];
        let message = line_total(&products, 9, 1).unwrap_err();
        assert!(message.contains("unknown id 9"), "{message}");
    }

    #[test]
    fn line_total_overflows() {
        let products = vec![Product {
            id: 1,
            title: String::from("mug"),
            price_cents: u32::MAX,
        }];
        let message = line_total(&products, 1, 2).unwrap_err();
        assert!(message.contains("overflowed"), "{message}");
    }
}
