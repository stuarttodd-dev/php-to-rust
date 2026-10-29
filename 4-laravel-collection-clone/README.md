# 4 - Laravel-style collection

![php-to-rust logo](../images/php-to-rust.png)

Companion to the Closures and iterators exercise: a generic `Collection<T>` with `map`, `filter`, `pluck`, `sum_by`, and `first_where`. Methods take `impl FnMut`. `pluck` borrows `&self`.

## Starter

`src/main.rs` is the starter. Bodies are `todo!`. Replace them, then:

```bash
cargo test
cargo run
```

## Solution

`solution/main.rs` is the finished file. Copy it over `src/main.rs` to check:

```bash
cp solution/main.rs src/main.rs
cargo test
cargo run
```

Expected `cargo run` output:

```text
[2, 4]
mug
1400
1000
```
