use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::process;

#[derive(Debug, Default)]
struct FileStats {
    lines: usize,
    words: usize,
    chars: usize,
}

fn main() -> io::Result<()> {
    let file_path_str = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: my_wc <file_path>");
        process::exit(1);
    });
    let file_path = Path::new(&file_path_str);
    let reader = BufReader::new(File::open(file_path)?);
    let mut stats = FileStats::default();

    for line_result in reader.lines() {
        let line = line_result?;
        stats.lines += 1;
        stats.words += line.split_whitespace().count();
        stats.chars += line.chars().count();
    }
    let file_size_bytes = fs::metadata(file_path)?.len();
    println!("Lines: {}  Words: {}  Chars: {}  Bytes: {}",
             stats.lines, stats.words, stats.chars, file_size_bytes);
    Ok(())
}
