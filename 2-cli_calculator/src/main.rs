use std::io;

fn main() {
    println!("Welcome to the Rust CLI Calculator!");

    let num1 = get_number("Enter the first number: ");
    let num2 = get_number("Enter the second number: ");

    println!("Enter an operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read input");
    let operation = operation.trim();

    let result = match operation {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero is not allowed.");
                None
            }
        }
        _ => {
            println!("Invalid operation. Please enter +, -, *, or /.");
            None
        }
    };

    if let Some(res) = result {
        println!("Result: {}", res);
    }
}

fn get_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number. Please enter a valid numeric value."),
        }
    }
}
