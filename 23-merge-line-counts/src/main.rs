use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn count_lines_in_file(path: &Path) -> io::Result<usize> {
    let f = File::open(path)?;
    let mut n = 0;
    for line in BufReader::new(f).lines() {
        let line = line?;
        if !line.trim().is_empty() {
            n += 1;
        }
    }
    Ok(n)
}

fn merge_line_counts(paths: &[&str]) -> io::Result<usize> {
    let mut total = 0usize;
    for p in paths {
        let path = Path::new(p);
        match count_lines_in_file(path) {
            Ok(n) => total += n,
            Err(e) => eprintln!("skip {}: {}", p, e),
        }
    }
    Ok(total)
}

fn main() -> io::Result<()> {
    let paths = ["data1.txt", "data2.txt"];
    println!("lines={}", merge_line_counts(&paths)?);
    Ok(())
}
