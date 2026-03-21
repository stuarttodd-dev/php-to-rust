use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn count_words_in_text(text: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for raw in text.split_whitespace() {
        let word = raw
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if word.is_empty() {
            continue;
        }
        *map.entry(word).or_insert(0) += 1;
    }
    map
}

fn merge_counts(into: &mut HashMap<String, u32>, from: HashMap<String, u32>) {
    for (word, count) in from {
        *into.entry(word).or_insert(0) += count;
    }
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    if paths.is_empty() {
        eprintln!("Usage: cargo run -- <file1> <file2> ...");
        std::process::exit(1);
    }

    let (tx, rx) = mpsc::channel::<Result<HashMap<String, u32>, String>>();
    let mut handles = Vec::new();

    for path in paths {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            let result = match fs::read_to_string(&path) {
                Ok(content) => Ok(count_words_in_text(&content)),
                Err(err) => Err(format!("{}: {}", path, err)),
            };
            let _ = tx_clone.send(result);
        }));
    }
    drop(tx);

    let mut global_counts = HashMap::new();
    for message in rx {
        match message {
            Ok(local_counts) => merge_counts(&mut global_counts, local_counts),
            Err(err_msg) => eprintln!("Worker error: {}", err_msg),
        }
    }

    for h in handles {
        h.join().unwrap();
    }

    let mut entries: Vec<_> = global_counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    println!("Top 10 words:");
    for (word, count) in entries.into_iter().take(10) {
        println!("{}: {}", word, count);
    }
}
