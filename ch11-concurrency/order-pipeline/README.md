# Order pipeline (chapter 11)

Matches the **Exercise: Order pipeline** lesson: worker threads take orders from a channel, reserve stock under one `Mutex` guard, and sort results by order id.

```console
cargo test
```

`src/main.rs` is the starter (`todo!()`). `solution/main.rs` is the filled-in program.
