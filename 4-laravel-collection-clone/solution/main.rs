struct Product {
    title: String,
    price_cents: u32,
}

struct Collection<T> {
    items: Vec<T>,
}

#[derive(Debug)]
enum BasketError {
    Overflow,
}

impl<T> Collection<T> {
    fn map<U>(self, f: impl FnMut(T) -> U) -> Collection<U> {
        Collection {
            items: self.items.into_iter().map(f).collect(),
        }
    }

    fn filter(self, mut f: impl FnMut(&T) -> bool) -> Collection<T> {
        Collection {
            items: self.items.into_iter().filter(|item| f(item)).collect(),
        }
    }

    fn pluck<U>(&self, mut f: impl FnMut(&T) -> U) -> Collection<U> {
        Collection {
            items: self.items.iter().map(&mut f).collect(),
        }
    }

    fn sum_by(&self, mut f: impl FnMut(&T) -> u32) -> Result<u32, BasketError> {
        self.items.iter().try_fold(0u32, |acc, item| {
            acc.checked_add(f(item)).ok_or(BasketError::Overflow)
        })
    }

    fn first_where(&self, mut f: impl FnMut(&T) -> bool) -> Option<&T> {
        self.items.iter().find(|item| f(item))
    }
}

fn main() {
    let doubled = Collection { items: vec![1, 2] }.map(|n| n * 2);
    println!("{:?}", doubled.items);
    let products = Collection {
        items: vec![
            Product {
                title: String::from("mug"),
                price_cents: 1000,
            },
            Product {
                title: String::from("spoon"),
                price_cents: 400,
            },
        ],
    };
    let kept = products.filter(|product| product.price_cents > 500);
    let titles = kept.pluck(|product| product.title.clone());
    println!("{}", titles.items[0]);
    let stock = Collection {
        items: vec![
            Product {
                title: String::from("mug"),
                price_cents: 1000,
            },
            Product {
                title: String::from("spoon"),
                price_cents: 400,
            },
        ],
    };
    match stock.sum_by(|product| product.price_cents) {
        Ok(cents) => println!("{cents}"),
        Err(BasketError::Overflow) => println!("overflow"),
    }
    let found = stock.first_where(|product| product.title == "mug");
    println!("{}", found.unwrap().price_cents);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn products() -> Collection<Product> {
        Collection {
            items: vec![
                Product {
                    title: String::from("mug"),
                    price_cents: 1000,
                },
                Product {
                    title: String::from("spoon"),
                    price_cents: 400,
                },
            ],
        }
    }

    #[test]
    fn map_doubles_a_list() {
        let nums = Collection {
            items: vec![1, 2, 3],
        };
        assert_eq!(nums.map(|n| n * 2).items, vec![2, 4, 6]);
    }

    #[test]
    fn filter_keeps_a_price_above_a_line() {
        let kept = products().filter(|product| product.price_cents > 500);
        assert_eq!(kept.items.len(), 1);
        assert_eq!(kept.items[0].title, "mug");
    }

    #[test]
    fn pluck_collects_titles() {
        let titles = products().pluck(|product| product.title.clone());
        assert_eq!(
            titles.items,
            vec![String::from("mug"), String::from("spoon")]
        );
    }

    #[test]
    fn sum_by_adds_price_cents() {
        let total = products().sum_by(|product| product.price_cents).unwrap();
        assert_eq!(total, 1400);
    }

    #[test]
    fn first_where_finds_the_mug() {
        let stock = products();
        let found = stock.first_where(|product| product.title == "mug");
        assert_eq!(found.unwrap().price_cents, 1000);
    }

    #[test]
    fn empty_first_where_is_none() {
        let empty: Collection<Product> = Collection { items: vec![] };
        assert!(
            empty
                .first_where(|product| product.title == "mug")
                .is_none()
        );
    }

    #[test]
    fn filter_accepts_fnmut() {
        let mut calls = 0;
        let kept = products().filter(|product| {
            calls += 1;
            product.price_cents > 500
        });
        assert_eq!(kept.items.len(), 1);
        assert_eq!(calls, 2);
    }
}
