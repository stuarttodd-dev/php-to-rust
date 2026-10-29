# 3 - User Input Classifier

![php-to-rust logo](../images/php-to-rust.png)

Classify a `Vec<Option<i32>>` with `match`, a guard for the large band, and a `BTreeMap` tally. Companion to the Rust basics Input classifier lesson.

## Run

From this folder:

```bash
cargo test
cargo run
```

Expected `cargo run` output (sorted by `BTreeMap` key):

```text
even 2
large 1
missing 2
negative 1
odd 2
positive 1
zero 1
```
