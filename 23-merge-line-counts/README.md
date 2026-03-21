# 23 - Merge Line Counts

![php-to-rust logo](../images/php-to-rust.png)

This example demonstrates merging line counts from multiple files, skipping empty lines and handling errors gracefully.

Features:
- Count non-empty lines in multiple files
- Merge counts into a total
- Skip files on error and continue processing

## Run

From this folder:

```bash
cargo run
```

Expected output:

```
lines=9
```

This counts the non-empty lines from `data1.txt` (6 lines) and `data2.txt` (3 lines) for a total of 9 lines.

## Files Included

- `data1.txt`: Contains 6 non-empty lines
- `data2.txt`: Contains 3 non-empty lines
