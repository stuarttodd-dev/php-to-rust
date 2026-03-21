# 22 - CLI File Operations

![php-to-rust logo](../images/php-to-rust.png)

This example implements a command-line file statistics tool (similar to `wc`), counting lines, words, characters, and bytes in a file.

Features:
- Command-line argument parsing
- File reading and metadata
- Line/word/character statistics

## Build

From this folder:

```bash
cargo build --release
```

Binary will be at `target/release/cmd-file-operations`.

## Testing

### 1. Create a Test File

```bash
cd /Users/stuart/PhpstormProjects/php-to-rust/22-cmd-file-operations
echo "Hello, world!
This is a test file.
Rust file operations rock!" > test.txt
```

### 2. Run with the Test File

```bash
cargo run -- test.txt
```

Expected output:

```
Lines: 3  Words: 7  Chars: 59  Bytes: 59
```

### 3. Test with Any Existing File

```bash
cargo run -- /path/to/any/file.txt
```

### 4. Test Error Handling (Missing Argument)

```bash
cargo run
```

Expected output:

```
Usage: my_wc <file_path>
```

The tool requires a file path as a command-line argument and reports file statistics.
