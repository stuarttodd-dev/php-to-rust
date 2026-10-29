#[macro_export]
macro_rules! money {
    ($pounds:expr) => {
        $crate::Money($pounds * 100)
    };
    ($pounds:expr, $pence:expr) => {
        $crate::Money($pounds * 100 + $pence)
    };
}

#[macro_export]
macro_rules! routes {
    ( $( $method:ident $path:literal ),* $(,)? ) => {
        [ $( (stringify!($method), $path) ),* ]
    };
}

/// Cents as an integer. Re-exported so `$crate::Money` works from other crates
/// after `#[macro_export]` lifts `money!` to the crate root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Money(pub u32);

pub use Money as MoneyCents;

#[cfg(test)]
mod tests {
    use super::Money;

    #[test]
    fn money_single_arm() {
        let mug: Money = crate::money!(10);
        assert_eq!(mug.0, 1000);
    }

    #[test]
    fn money_two_arms() {
        let bowl: Money = crate::money!(25, 0);
        assert_eq!(bowl.0, 2500);
    }

    #[test]
    fn routes_table() {
        let table = crate::routes!(GET "/health", POST "/baskets");
        assert_eq!(table.len(), 2);
        assert_eq!(table[0], ("GET", "/health"));
    }
}
