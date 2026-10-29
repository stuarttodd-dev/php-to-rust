use lifetimes_practice::Parser;

fn main() {
    let mut parser = Parser::new("mug bowl");
    println!("{}", parser.next_word().unwrap());
    println!("{}", parser.next_word().unwrap());
}
