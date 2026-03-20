mod error;
mod logging;
mod parse;
mod process;

fn main() {
    let sample = "a=1\nb=2\n# comment\nc=hello\n=emptykey\nnotanassignment\n";
    println!("sum = {}", process::sum_integer_values(sample));
}
