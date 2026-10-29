#[test]
fn no_code_on_a_mug_is_1000() {
    assert_eq!(shop_front::mug_total("", 1), Some(1000));
}

#[test]
fn save10_on_a_mug_is_900() {
    assert_eq!(shop_front::mug_total("SAVE10", 1), Some(900));
}

#[test]
fn take100_on_a_mug_is_900() {
    assert_eq!(shop_front::mug_total("TAKE100", 1), Some(900));
}

#[test]
fn unknown_code_is_none() {
    assert_eq!(shop_front::mug_total("NOPE", 1), None);
}
