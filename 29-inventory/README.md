# 29-inventory

![php-to-rust logo](../images/php-to-rust.png)

Small CLI that walks a directory tree (up to a fixed max depth), collects **relative file paths and byte sizes**, then prints a **summary grouped by file extension** (total bytes and file count per extension). Uses **`Path`**, **`PathBuf`**, and `std::fs`.

The grouping logic lives in **`summarize_by_extension`**, which is covered by unit tests in the same file.

## Prerequisites

- Rust toolchain (this crate uses **edition 2024**; use a recent stable or nightly that supports it).

## Run

From this directory:

```bash
cargo run
```

Scans the **current directory** (`.`). Pass a different root:

```bash
cargo run -- /path/to/directory
```

If the path is missing or not a directory, the program prints an error and exits with code **1**.

Output is a table: `extension`, `bytes`, `files`, plus a **TOTAL** row. Files with no extension are grouped under **`(no ext)`**. Read errors on individual entries are reported as warnings; the walk continues.

## Tests

```bash
cargo test
```

Runs tests for `summarize_by_extension` (extension bucketing, including the no-extension case).

## Project layout

| Path | Purpose |
|------|--------|
| `src/main.rs` | `summarize_by_extension`, directory walk, report printing, `main`, `#[cfg(test)]` module |

## Related lesson

- [`PathBuf` and `Path` (systems track)](../learn/sections/rust-for-systems-development/systems-pathbuf-and-path.md)
