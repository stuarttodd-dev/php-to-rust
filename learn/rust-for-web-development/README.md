# Rust for web development — lesson copy for your LMS

This folder is **source text** for the course site (e.g. `http://127.0.0.1:8088/learn/...`). It is **not** served by this repo; paste or import into whatever powers your `/learn` app.

## Recommended section order

| Order | File | Suggested URL slug | Notes |
|------:|------|--------------------|--------|
| 1 | [01-rust-wasm-frontend-options.md](./01-rust-wasm-frontend-options.md) | `rust-wasm-frontend-options` | One lesson, **three** frameworks, **no** embedded video. |
| 2 | [02-build-simple-front-end.md](./02-build-simple-front-end.md) | `build-simple-rust-front-end` | Pick one stack; static shell first, API later. |

## Retire the old lesson

Remove or redirect:

- **Old slug:** `yew-components-and-demo-video`  
- **Reason:** Course is no longer Yew-only and no longer centred on a demo video.

Point any “next” links from the prior Dioxus overview lesson at **`rust-wasm-frontend-options`** if that fits your narrative, or order these two **before** framework-specific deep dives.

## Repo projects referenced

- **`28-shop-front`** — Dioxus + Trunk static shop shell (reference implementation for lesson 2).
- **`27-shop-api`** — Backend used **after** the static UI exists.
