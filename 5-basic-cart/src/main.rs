struct Cart {
    items: Vec<String>,
}

impl Cart {
    fn new() -> Cart {
        Cart { items: Vec::new() }
    }

    fn add(&mut self, name: &str) {
        self.items.push(name.to_string());
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn print_items(&self) {
        for item in &self.items {
            println!(" - {}", item);
        }
    }
}

fn prefix_items(cart: &mut Cart, prefix: &str) {
    for item in cart.items.iter_mut() {
        *item = format!("{}{}", prefix, item);
    }
}

fn checkout(cart: Cart) -> String {
    format!("checked out {} items", cart.items.len())
}

fn main() {
    let mut cart = Cart::new();
    cart.add("milk");
    println!("count: {}", cart.len());
    cart.add("bread");
    println!("count: {}", cart.len());
    cart.print_items();
    prefix_items(&mut cart, "[sale] ");
    cart.print_items();
    let msg = checkout(cart);
    println!("{}", msg);
}
