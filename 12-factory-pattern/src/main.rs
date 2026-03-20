trait Animal {
    fn speak(&self) -> &'static str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) -> &'static str {
        "woof"
    }
}
impl Animal for Cat {
    fn speak(&self) -> &'static str {
        "meow"
    }
}

#[derive(Clone, Copy)]
enum PetKind {
    Dog,
    Cat,
}

struct PetFactory;

impl PetFactory {
    fn create(kind: PetKind) -> Box<dyn Animal> {
        match kind {
            PetKind::Dog => Box::new(Dog),
            PetKind::Cat => Box::new(Cat),
        }
    }
}

fn main() {
    let d = PetFactory::create(PetKind::Dog);
    let c = PetFactory::create(PetKind::Cat);
    println!("{}", d.speak());
    println!("{}", c.speak());
}