# 20 - Shared Document Views

![php-to-rust logo](../images/php-to-rust.png)

This example demonstrates shared ownership in Rust using `Rc` (Reference Counted) smart pointers, allowing multiple components to share the same document without copying.

## Run

From this folder:

```bash
cargo run
```

Expected output shows document preview, word count, and reference counts before/after dropping a viewer.
