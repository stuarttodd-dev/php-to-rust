use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// extension -> (total_bytes, file_count)
pub fn summarize_by_extension(files: &[(String, u64)]) -> HashMap<String, (u64, usize)> {
    let mut m = HashMap::new();
    for (path_str, size) in files {
        let path = Path::new(path_str);
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("(no ext)")
            .to_string();
        let entry = m.entry(ext).or_insert((0_u64, 0_usize));
        entry.0 += size;
        entry.1 += 1;
    }
    m
}

fn walk_files(
    root: &Path,
    current: &Path,
    depth: usize,
    max_depth: usize,
    out: &mut Vec<(String, u64)>,
) -> std::io::Result<()> {
    if depth > max_depth {
        return Ok(());
    }

    for entry in fs::read_dir(current)? {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("warning: {}", e);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            if let Err(e) = walk_files(root, &path, depth + 1, max_depth, out) {
                eprintln!("warning: {}: {}", path.display(), e);
            }
        } else if path.is_file() {
            match fs::metadata(&path) {
                Ok(meta) => {
                    let len = meta.len();
                    let rel = path.strip_prefix(root).unwrap_or(&path);
                    out.push((rel.display().to_string(), len));
                }
                Err(e) => eprintln!("warning: {}: {}", path.display(), e),
            }
        }
    }

    Ok(())
}

fn print_report(summary: &HashMap<String, (u64, usize)>) {
    let mut rows: Vec<_> = summary.iter().collect();
    rows.sort_by(|a, b| a.0.cmp(b.0));

    println!("{:20} {:>12} {:>10}", "extension", "bytes", "files");
    println!("{}", "-".repeat(42));

    let mut total_bytes = 0_u64;
    let mut total_files = 0_usize;
    for (ext, (bytes, count)) in rows {
        println!("{:20} {:>12} {:>10}", ext, bytes, count);
        total_bytes += bytes;
        total_files += count;
    }

    println!("{}", "-".repeat(42));
    println!("{:20} {:>12} {:>10}", "TOTAL", total_bytes, total_files);
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let root_arg = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let root = PathBuf::from(&root_arg);

    if !root.is_dir() {
        return Err(format!("not a directory: {}", root.display()).into());
    }

    const MAX_DEPTH: usize = 32;
    let mut files = Vec::new();
    walk_files(root.as_path(), root.as_path(), 0, MAX_DEPTH, &mut files)?;

    let summary = summarize_by_extension(&files);
    print_report(&summary);

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summarize_groups_by_extension() {
        let files = vec![
            ("src/main.rs".to_string(), 100),
            ("src/lib.rs".to_string(), 200),
            ("README.md".to_string(), 50),
        ];
        let s = summarize_by_extension(&files);
        assert_eq!(s.get("rs"), Some(&(300, 2)));
        assert_eq!(s.get("md"), Some(&(50, 1)));
    }

    #[test]
    fn no_extension_bucket() {
        let files = vec![("Dockerfile".to_string(), 10)];
        let s = summarize_by_extension(&files);
        assert_eq!(s.get("(no ext)"), Some(&(10, 1)));
    }
}
