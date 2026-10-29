# shop-front

Leptos **0.8** CSR shop page for the WebAssembly chapter. Lists products from `GET /products`, posts lines, and compares `mug_total("SAVE10", qty)` with `POST /baskets/{id}/discount`.

```bash
# API from ch12 must listen on 127.0.0.1:8090 with CORS for 8091
cargo test --test pricing
trunk serve --address 127.0.0.1 --port 8091
```

For the const-only list from A Rust frontend, drop the network calls and render the house catalog from a `const` in `ProductList`.
