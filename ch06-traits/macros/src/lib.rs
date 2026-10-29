#[macro_export]
macro_rules! money {
    ($pounds:expr) => {
        shop::Money($pounds * 100)
    };
    ($pounds:expr, $pence:expr) => {
        shop::Money($pounds * 100 + $pence)
    };
}

#[macro_export]
macro_rules! routes {
    ( $( $method:ident $path:literal ),* $(,)? ) => {
        [ $( (stringify!($method), $path) ),* ]
    };
}

#[cfg(test)]
mod tests {
    use shop::Money;

    #[test]
    fn money_single_arm() {
        let mug: Money = crate::money!(10);
        assert_eq!(mug.0, 1000);
    }

    #[test]
    fn routes_table() {
        let table = crate::routes!(GET "/health", POST "/baskets");
        assert_eq!(table.len(), 2);
        assert_eq!(table[0], ("GET", "/health"));
    }
}
