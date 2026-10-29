use std::collections::BTreeMap;

fn classify(n: i32) -> &'static str {
    match n {
        n if n >= 100 => "large",
        1..100 => "positive",
        0 => "zero",
        _ => "negative",
    }
}

fn parity(n: i32) -> &'static str {
    if n % 2 == 0 { "even" } else { "odd" }
}

fn summarize(inputs: &[Option<i32>]) -> BTreeMap<&'static str, i32> {
    let mut counts = BTreeMap::new();
    for input in inputs {
        match input {
            None => {
                *counts.entry("missing").or_insert(0) += 1;
            }
            Some(n) => {
                *counts.entry(classify(*n)).or_insert(0) += 1;
                *counts.entry(parity(*n)).or_insert(0) += 1;
            }
        }
    }
    counts
}

fn main() {
    let inputs: Vec<Option<i32>> = vec![Some(12), None, Some(0), Some(-3), Some(101), None];
    let counts = summarize(&inputs);
    for (label, count) in &counts {
        println!("{label} {count}");
    }
}

#[cfg(test)]
mod tests {
    use super::{classify, parity, summarize};

    #[test]
    fn classifies_sign_and_size() {
        assert_eq!(classify(12), "positive");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(-3), "negative");
        assert_eq!(classify(99), "positive");
        assert_eq!(classify(100), "large");
        assert_eq!(classify(101), "large");
    }

    #[test]
    fn classifies_parity() {
        assert_eq!(parity(12), "even");
        assert_eq!(parity(0), "even");
        assert_eq!(parity(-3), "odd");
    }

    #[test]
    fn counts_categories() {
        let inputs = vec![Some(12), None, Some(0), Some(-3), Some(101), None];
        let counts = summarize(&inputs);
        assert_eq!(counts.get("positive"), Some(&1));
        assert_eq!(counts.get("zero"), Some(&1));
        assert_eq!(counts.get("negative"), Some(&1));
        assert_eq!(counts.get("large"), Some(&1));
        assert_eq!(counts.get("missing"), Some(&2));
        assert_eq!(counts.get("even"), Some(&2));
        assert_eq!(counts.get("odd"), Some(&2));
    }
}
