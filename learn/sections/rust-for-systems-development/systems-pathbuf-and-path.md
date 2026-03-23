# `PathBuf` and `Path`

Rust represents file-system locations with two types in `std::path`:

- **`Path`** — borrowed view (like `str` for strings), often seen as `&Path`.
- **`PathBuf`** — owned, mutable path (like `String`).

You usually build or adjust paths with **`PathBuf`**, then pass **`&Path`** to APIs that only need to read them (`PathBuf` derefs to `Path`).

## Example: root directory argument

The [`29-inventory`](../../../29-inventory/) program reads an optional CLI argument (default `"."`), builds a **`PathBuf`**, and checks **`is_dir()`** before walking the tree:

```rust
let root_arg = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
let root = PathBuf::from(&root_arg);

if !root.is_dir() {
    return Err(format!("not a directory: {}", root.display()).into());
}
```

- **`display()`** prints a human-readable path (lossy for non-UTF-8 on some platforms).
- **`is_dir()`** asks the OS whether that path exists and is a directory (symlinks follow platform rules).

## Grouping by extension (pure logic)

The same crate exposes **`summarize_by_extension`**, which takes a list of `(path, size)` pairs and returns a map **extension → (total bytes, file count)**. That logic is covered by **`#[test]`** functions in `main.rs` (e.g. `summarize_groups_by_extension`, `no_extension_bucket`).

## Run it locally

```bash
cd 29-inventory
cargo run
cargo run -- /path/to/a/directory
cargo test
```

Use real paths on your machine; directory listings depend on your filesystem.

## See also

- [`std::path::PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
- [`std::path::Path`](https://doc.rust-lang.org/std/path/struct.Path.html)
