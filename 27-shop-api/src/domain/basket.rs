//! Shopping basket: `BasketLine`, `Basket`, and in-memory line rules (domain only).

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasketLine {
    pub product_id: u64,
    pub quantity: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Basket {
    pub lines: Vec<BasketLine>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasketError {
    ZeroOrNegativeQty,
}

impl std::fmt::Display for BasketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ZeroOrNegativeQty => "quantity must be greater than zero",
        })
    }
}

impl std::error::Error for BasketError {}

impl Basket {
    fn validate_qty(qty: u32) -> Result<(), BasketError> {
        if qty == 0 {
            return Err(BasketError::ZeroOrNegativeQty);
        }
        Ok(())
    }

    pub fn add(&mut self, product_id: u64, qty: u32) -> Result<(), BasketError> {
        Self::validate_qty(qty)?;
        if let Some(line) = self.lines.iter_mut().find(|l| l.product_id == product_id) {
            line.quantity += qty;
        } else {
            self.lines.push(BasketLine {
                product_id,
                quantity: qty,
            });
        }
        Ok(())
    }

    pub fn set_line(&mut self, product_id: u64, qty: u32) -> Result<(), BasketError> {
        Self::validate_qty(qty)?;
        if let Some(line) = self.lines.iter_mut().find(|l| l.product_id == product_id) {
            line.quantity = qty;
        } else {
            self.lines.push(BasketLine {
                product_id,
                quantity: qty,
            });
        }
        Ok(())
    }

    pub fn remove_line(&mut self, product_id: u64) -> bool {
        let before = self.lines.len();
        self.lines.retain(|l| l.product_id != product_id);
        before != self.lines.len()
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn total_items(&self) -> u32 {
        self.lines.iter().map(|l| l.quantity).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qty(c: &Basket, product_id: u64) -> Option<u32> {
        c.lines
            .iter()
            .find(|l| l.product_id == product_id)
            .map(|l| l.quantity)
    }

    #[test]
    fn add_accumulates_quantity_for_same_product() {
        let mut c = Basket::default();
        c.add(1, 2).unwrap();
        c.add(1, 1).unwrap();
        assert_eq!(qty(&c, 1), Some(3));
        assert_eq!(c.total_items(), 3);
    }

    #[test]
    fn add_rejects_zero_quantity() {
        let mut c = Basket::default();
        assert_eq!(c.add(1, 0).unwrap_err(), BasketError::ZeroOrNegativeQty);
    }

    #[test]
    fn set_line_replaces_existing_line() {
        let mut c = Basket::default();
        c.add(1, 5).unwrap();
        c.set_line(1, 2).unwrap();
        assert_eq!(qty(&c, 1), Some(2));
    }

    #[test]
    fn remove_line_drops_product() {
        let mut c = Basket::default();
        c.add(1, 2).unwrap();
        assert!(c.remove_line(1));
        assert_eq!(qty(&c, 1), None);
    }

    #[test]
    fn clear_removes_all_lines() {
        let mut c = Basket::default();
        c.add(1, 1).unwrap();
        c.clear();
        assert!(c.lines.is_empty());
    }
}
