fn add_item(basket: &mut Vec<String>, item: &str) {
    basket.push(item.to_string());
}

fn total_len(items: &[String]) -> usize {
    let mut total = 0;
    for item in items {
        total += item.len();
    }
    total
}

fn longest_item(items: &[String]) -> Option<&str> {
    let mut best: Option<&str> = None;
    for item in items {
        best = match best {
            None => Some(item),
            Some(current) if item.len() > current.len() => Some(item),
            Some(current) => Some(current),
        };
    }
    best
}

fn remove_item(basket: &mut Vec<String>, item: &str) -> Option<String> {
    for i in 0..basket.len() {
        if basket[i] == item {
            return Some(basket.remove(i));
        }
    }
    None
}

fn main() {
    let mut basket = vec![String::from("ciabatta"), String::from("espresso")];
    let first = longest_item(&basket);
    println!("{}", first.unwrap_or("none"));
    add_item(&mut basket, "pesto");
    println!("{}", total_len(&basket));
    let removed = remove_item(&mut basket, "espresso");
    println!("{removed:?}");
    println!("{basket:?}");
}

#[cfg(test)]
mod tests {
    use super::{add_item, longest_item, remove_item, total_len};

    #[test]
    fn adds_an_owned_copy_of_the_name() {
        let mut basket = Vec::new();
        let item = "ciabatta";
        add_item(&mut basket, item);
        assert_eq!(basket, vec![String::from("ciabatta")]);
        assert_eq!(item, "ciabatta");
    }

    #[test]
    fn sums_byte_lengths() {
        let items = vec![String::from("ciabatta"), String::from("pesto")];
        assert_eq!(total_len(&items), 13);
        assert_eq!(total_len(&[]), 0);
    }

    #[test]
    fn borrows_the_longest_item() {
        let items = vec![
            String::from("pesto"),
            String::from("ciabatta"),
            String::from("bread"),
        ];
        assert_eq!(longest_item(&items), Some("ciabatta"));
        assert_eq!(longest_item(&[]), None);
        let tied = vec![String::from("pesto"), String::from("bread")];
        assert_eq!(longest_item(&tied), Some("pesto"));
    }

    #[test]
    fn removes_an_owned_string() {
        let mut basket = vec![
            String::from("ciabatta"),
            String::from("espresso"),
            String::from("pesto"),
        ];
        assert_eq!(
            remove_item(&mut basket, "espresso"),
            Some(String::from("espresso"))
        );
        assert_eq!(
            basket,
            vec![String::from("ciabatta"), String::from("pesto")]
        );
        assert_eq!(remove_item(&mut basket, "missing"), None);
    }
}
