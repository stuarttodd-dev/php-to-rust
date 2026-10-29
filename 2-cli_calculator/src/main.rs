use std::io::{self, Write};

/// Preview: `Option` is a value that is either `Some(number)` or `None`.
/// The Option chapter covers it properly. Integers divide by truncation: `7 / 2` is `3`.
fn apply(operation: &str, left: i32, right: i32) -> Option<i32> {
    match operation {
        "+" => left.checked_add(right),
        "-" => left.checked_sub(right),
        "*" => left.checked_mul(right),
        "/" => left.checked_div(right),
        _ => None,
    }
}

fn prompt(label: &str) -> Option<String> {
    print!("{label}");
    io::stdout().flush().expect("stdout should flush");
    let mut line = String::new();
    let bytes = io::stdin()
        .read_line(&mut line)
        .expect("stdin should be readable");
    if bytes == 0 {
        return None;
    }
    Some(line.trim().to_string())
}

fn main() {
    println!("Welcome to the Rust CLI calculator.");
    let Some(raw_left) = prompt("First number: ") else {
        return;
    };
    let Some(raw_right) = prompt("Second number: ") else {
        return;
    };
    let Some(operation) = prompt("Operation (+, -, *, /): ") else {
        return;
    };
    let Ok(left) = raw_left.parse::<i32>() else {
        println!("The first value should be a whole number.");
        return;
    };
    let Ok(right) = raw_right.parse::<i32>() else {
        println!("The second value should be a whole number.");
        return;
    };
    if let Some(result) = apply(&operation, left, right) {
        println!("Result: {result}");
    } else {
        println!("Use + - * or /, and do not overflow or divide by zero.");
    }
}

#[cfg(test)]
mod tests {
    use super::apply;

    #[test]
    fn adds() {
        assert_eq!(apply("+", 10, 4), Some(14));
    }

    #[test]
    fn integer_division_truncates() {
        assert_eq!(apply("/", 7, 2), Some(3));
    }

    #[test]
    fn rejects_divide_by_zero() {
        assert_eq!(apply("/", 7, 0), None);
    }

    #[test]
    fn overflow_is_none() {
        assert_eq!(apply("+", i32::MAX, 1), None);
    }
}
