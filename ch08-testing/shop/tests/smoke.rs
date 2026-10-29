mod common;

use shop::Money;

#[test]
fn totals_the_mug() {
    let (basket, catalog) = common::mug();
    assert_eq!(basket.total(&catalog).unwrap(), Money(1000));
}
