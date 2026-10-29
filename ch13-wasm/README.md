# ch13-wasm

Leptos **0.8** CSR examples for the WebAssembly chapter: shared pricing through `wasm-pack`, then a Trunk-served shop front that calls the Axum API from [`ch12-building-an-api`](../ch12-building-an-api/README.md).

| Path | Lesson |
|---|---|
| [`shop-pricing/`](shop-pricing/) | Sharing shop logic |
| [`shop-front/`](shop-front/) | A Rust frontend / Calling the API / Basket page |

`shop` and `discounts` come from the chapter 12 workspace by path.

## Prerequisites

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
cargo install trunk
```

## Alternative framework

[`28-shop-ui`](../28-shop-ui/README.md) is a **Dioxus 0.7** app from an earlier draft of this chapter. It is not the Leptos path the lessons teach.
