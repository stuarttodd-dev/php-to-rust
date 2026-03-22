//! Catalog product type, validation, sample data, and lookup (domain only).

#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Product {
    #[sqlx(try_from = "i64")]
    pub id: u64,
    pub title: String,
    #[sqlx(try_from = "i32")]
    pub price_cents: u32,
    /// Wholesale / internal cost — not exposed on public JSON (`ProductResponse`).
    #[sqlx(try_from = "i32")]
    pub cost_cents: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProductValidationError {
    EmptyTitle,
    ZeroPrice,
}

impl std::fmt::Display for ProductValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::EmptyTitle => "title must not be empty",
            Self::ZeroPrice => "price_cents must be greater than zero",
        })
    }
}

impl std::error::Error for ProductValidationError {}

impl Product {
    pub fn try_new(
        id: u64,
        title: impl Into<String>,
        price_cents: u32,
        cost_cents: u32,
    ) -> Result<Self, ProductValidationError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(ProductValidationError::EmptyTitle);
        }
        if price_cents == 0 {
            return Err(ProductValidationError::ZeroPrice);
        }
        Ok(Self {
            id,
            title: title.trim().to_string(),
            price_cents,
            cost_cents,
        })
    }
}

pub fn catalog_sample() -> Vec<Product> {
    vec![
        Product::try_new(1, "Rust mug", 1299, 400).expect("sample data is valid"),
        Product::try_new(2, "Ferris sticker", 399, 120).expect("sample data is valid"),
    ]
}

pub fn find_by_id<'a>(catalog: &'a [Product], id: u64) -> Option<&'a Product> {
    catalog.iter().find(|p| p.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_trims_surrounding_whitespace_on_title() {
        let p = Product::try_new(1, "  Rust mug  ", 1299, 0).unwrap();
        assert_eq!(p.title, "Rust mug");
    }

    #[test]
    fn try_new_rejects_whitespace_only_title() {
        assert_eq!(
            Product::try_new(1, "", 100, 0).unwrap_err(),
            ProductValidationError::EmptyTitle
        );
        assert_eq!(
            Product::try_new(1, "   ", 100, 0).unwrap_err(),
            ProductValidationError::EmptyTitle
        );
    }

    #[test]
    fn try_new_rejects_zero_price() {
        assert_eq!(
            Product::try_new(1, "Valid title", 0, 0).unwrap_err(),
            ProductValidationError::ZeroPrice
        );
    }

    #[test]
    fn sample_catalog_has_two_items() {
        assert_eq!(catalog_sample().len(), 2);
    }

    #[test]
    fn find_by_id_finds_or_misses() {
        let c = catalog_sample();
        assert_eq!(find_by_id(&c, 1).map(|p| p.title.as_str()), Some("Rust mug"));
        assert!(find_by_id(&c, 999).is_none());
    }
}
