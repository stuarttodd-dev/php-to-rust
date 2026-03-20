mod error;
mod logging;
mod parse;
mod process;

fn main() {
    env_logger::init();

    let sample = "a=1\nb=4\n# comment\nc=hello\n=emptykey\nnotanassignment\n";
    println!("sum = {}", process::sum_integer_values(sample));
}
