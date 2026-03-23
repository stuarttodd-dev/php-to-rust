# Build a simple Rust front end (one stack)

**Suggested LMS slug:** `build-simple-rust-front-end`  
**Prerequisite:** [Rust WASM frontend options](./01-rust-wasm-frontend-options.md) (choose your stack).

## Purpose

Implement a **minimal** browser UI: **layout** (header, main, footer), **two or three routes** (e.g. catalog, product, checkout), and **static placeholder** copy. No API, no basket logic yet—only structure and styling so the next step is “plug in `fetch`”.

## If you follow the course repo (Dioxus + Trunk)

The worked example lives in **`28-shop-front`**:

- `#[derive(Routable)]` + shared **`AppLayout`** + **`Outlet`**
- Static product cards, a product page keyed by route `id`, a checkout table with hard-coded rows
- **`assets/styles.css`** linked from `index.html` via Trunk
- **`Trunk.toml`** proxies `/api/` to **`27-shop-api`** on port **8090** so later you can call `fetch("/api/...")` same-origin

See also **`28-shop-front/LESSON-shop-pages-after-dioxus-overview.md`** for step-by-step goals that match this lesson.

## If you chose Yew or Leptos instead

Keep the **same product goals**:

1. One **layout** with nav (including a **Checkout** link).
2. **Catalog** route: grid of items (static text is fine).
3. **Product** route: detail + visually obvious “Add to basket” (can be a non-button placeholder at first).
4. **Checkout** route: table + subtotal (static rows).

Implement with that framework’s **router** and **components**; use its docs for the exact macros and project layout.

## Done when

- [ ] I can navigate between all pages without a full reload (SPA routing).
- [ ] Header and footer are shared across routes.
- [ ] Styling is intentional (not browser default), even if simple.
- [ ] I know where HTTP calls will go **next** (endpoints and basket id), without implementing them yet.
