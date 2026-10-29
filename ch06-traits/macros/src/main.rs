use learn_macros::{money, routes};
use shop::Money;

fn main() {
    let mug: Money = money!(10);
    let bowl: Money = money!(25, 0);
    println!("{} {}", mug.0, bowl.0);

    for (method, path) in routes!(GET "/health", GET "/products") {
        println!("{method} {path}");
    }
}
