struct MiniCollection {
    items: Vec<i32>,
}

impl MiniCollection {
    fn new(items: Vec<i32>) -> Self {
        Self { items }
    }

    fn map<F>(self, f: F) -> Self
    where
        F: FnMut(i32) -> i32,
    {
        Self {
            items: self.items.into_iter().map(f).collect(),
        }
    }

    fn filter<P>(self, mut pred: P) -> Self
    where
        P: FnMut(&i32) -> bool,
    {
        Self {
            items: self.items.into_iter().filter(|x| pred(x)).collect(),
        }
    }

    fn fold<T, F>(self, init: T, f: F) -> T
    where
        F: FnMut(T, i32) -> T,
    {
        self.items.into_iter().fold(init, f)
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(i32),
    {
        self.items.into_iter().for_each(f);
    }
}

fn main() {
    let c = MiniCollection::new(vec![1, -2, 3, -4, 5]);
    let piped = c.map(|x| x * 2).filter(|&x| x > 0);
    let sum = piped.fold(0, |acc, x| acc + x);
    println!("sum after map*2 and filter>0: {}", sum);

    MiniCollection::new(vec![10, 20])
        .map(|x| x + 1)
        .for_each(|x| println!("item: {}", x));
}
