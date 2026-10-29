# shop-pricing

Shared `mug_total` for the Sharing shop logic lesson. Calls `discounts::rule_for` so `SAVE10` and `TAKE100` match the API.

```bash
cargo test --test pricing
wasm-pack build --target web
python3 -m http.server 8091
```

Open `http://127.0.0.1:8091/`. The body is `900`.
