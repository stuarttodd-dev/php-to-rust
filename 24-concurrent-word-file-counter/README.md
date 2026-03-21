# 24 - Concurrent Word File Counter

![php-to-rust logo](../images/php-to-rust.png)

This example demonstrates concurrent word counting across multiple files using Rust's threading and message passing.

Features:
- Concurrent file processing with threads
- Word frequency counting with HashMap
- Message passing with channels (mpsc)
- Merging results from multiple workers
- Top 10 word ranking

## Run

From this folder:

```bash
cargo run -- words.txt
```

Expected output shows top 10 words by frequency from the file.

## Test with Multiple Files

```bash
cargo run -- words.txt words.txt
```

This processes the same file twice concurrently and merges the counts.

## Usage

```bash
cargo run -- <file1> <file2> ...
```

Requires at least one file path argument. Processes files concurrently and reports word frequencies.