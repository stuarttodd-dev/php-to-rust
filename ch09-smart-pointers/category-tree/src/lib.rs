use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub struct Product {
    pub id: u64,
    pub title: String,
    pub price_cents: u32,
}

pub struct Category {
    pub name: String,
    pub products: Vec<Product>,
    pub children: RefCell<Vec<Rc<Category>>>,
    pub parent: RefCell<Weak<Category>>,
}

impl Category {
    pub fn breadcrumb(&self) -> String {
        todo!()
    }

    pub fn product_count(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shop() -> (Rc<Category>, Rc<Category>, Rc<Category>) {
        let home = Rc::new(Category {
            name: String::from("Home"),
            products: Vec::new(),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        });
        let kitchen = Rc::new(Category {
            name: String::from("Kitchen"),
            products: Vec::new(),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        });
        let mugs = Rc::new(Category {
            name: String::from("Mugs"),
            products: vec![Product {
                id: 1,
                title: String::from("mug"),
                price_cents: 1000,
            }],
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        });
        let bowls = Rc::new(Category {
            name: String::from("Bowls"),
            products: Vec::new(),
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
        });
        home.children.borrow_mut().push(Rc::clone(&kitchen));
        *kitchen.parent.borrow_mut() = Rc::downgrade(&home);
        kitchen.children.borrow_mut().push(Rc::clone(&mugs));
        *mugs.parent.borrow_mut() = Rc::downgrade(&kitchen);
        kitchen.children.borrow_mut().push(Rc::clone(&bowls));
        *bowls.parent.borrow_mut() = Rc::downgrade(&kitchen);
        (home, mugs, bowls)
    }

    #[test]
    fn mugs_breadcrumb_is_home_kitchen_mugs() {
        let (home, mugs, _bowls) = shop();
        assert_eq!(home.name, "Home");
        assert_eq!(mugs.name, "Mugs");
        assert_eq!(mugs.breadcrumb(), "Home > Kitchen > Mugs");
    }

    #[test]
    fn home_breadcrumb_is_home() {
        let (home, _mugs, _bowls) = shop();
        assert_eq!(home.breadcrumb(), "Home");
    }

    #[test]
    fn kitchen_counts_the_mug() {
        let (home, mugs, _bowls) = shop();
        assert_eq!(home.name, "Home");
        assert_eq!(mugs.products[0].id, 1);
        assert_eq!(mugs.products[0].title, "mug");
        assert_eq!(mugs.products[0].price_cents, 1000);
        let kitchen = mugs.parent.borrow().upgrade().unwrap();
        assert_eq!(kitchen.name, "Kitchen");
        assert_eq!(kitchen.products.len(), 0);
        assert_eq!(kitchen.product_count(), 1);
        assert_eq!(mugs.product_count(), 1);
    }

    #[test]
    fn home_counts_every_product_below() {
        let (home, mugs, bowls) = shop();
        assert_eq!(home.products.len(), 0);
        assert_eq!(mugs.products.len(), 1);
        assert_eq!(bowls.products.len(), 0);
        assert_eq!(home.product_count(), 1);
    }

    #[test]
    fn empty_leaf_counts_zero() {
        let (_home, _mugs, bowls) = shop();
        assert_eq!(bowls.name, "Bowls");
        assert_eq!(bowls.products.len(), 0);
        assert_eq!(bowls.product_count(), 0);
    }
}
