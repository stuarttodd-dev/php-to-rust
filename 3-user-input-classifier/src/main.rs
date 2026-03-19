fn classify_value(n: i32) -> (&'static str, &'static str) {
    let sign = if n > 0 {
        "positive"
    } else if n < 0 {
        "negative"
    } else {
        "zero"
    };

    let parity = if n % 2 == 0 { "even" } else { "odd" };
    (sign, parity)
}

fn main() {
    let inputs: Vec<Option<i32>> = vec![Some(12), None, Some(0), Some(-3), Some(101), None];

    let mut some_count = 0;
    let mut none_count = 0;
    let mut positive_count = 0;
    let mut even_count = 0;

    for input in inputs {
        match input {
            Some(n) => {
                some_count += 1;
                let (sign, parity) = classify_value(n);
                if n > 0 {
                    positive_count += 1;
                }
                if n % 2 == 0 {
                    even_count += 1;
                }

                println!("value={} => {}, {}", n, sign, parity);
            }
            None => {
                none_count += 1;
                println!("missing input");
            }
        }
    }

    println!("\nSummary:");
    println!("Some: {}", some_count);
    println!("None: {}", none_count);
    println!("Positive: {}", positive_count);
    println!("Even: {}", even_count);
}
