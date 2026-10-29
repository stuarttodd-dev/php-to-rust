use anyhow::{Context, Result};

fn read_order(path: &str) -> Result<String> {
    std::fs::read_to_string(path).context("reading order file")
}

fn main() -> Result<()> {
    let text = read_order("orders.txt")?;
    println!("{text}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read_order;

    #[test]
    fn missing_file_names_the_read() {
        let error = read_order("definitely-missing-orders.txt").unwrap_err();
        let message = format!("{error:#}");
        assert!(message.contains("reading order file"));
    }
}
