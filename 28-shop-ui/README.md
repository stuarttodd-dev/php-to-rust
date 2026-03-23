# 28-shop-ui

![php-to-rust logo](../images/php-to-rust.png)

Dioxus **0.7** web app: product catalog, product detail, basket (local storage + [`27-shop-api`](../27-shop-api/README.md) HTTP). Uses **`dx serve`** for the dev server (WASM in the browser).

## Prerequisites

- **Rust** (stable) with `wasm32-unknown-unknown`: `rustup target add wasm32-unknown-unknown`
- **Dioxus CLI**: [install `dx`](https://dioxuslabs.com/learn/0.7/getting_started) (e.g. `curl -sSL https://dioxus.dev/install.sh | sh` or `cargo install dioxus-cli --locked`)
- **Shop API** from [`27-shop-api`](../27-shop-api/README.md) reachable from your machine (default **`http://127.0.0.1:8090`**)

## 1. Run the API (and database)

The UI calls the API from the browser (cross-origin), so the API must be running with CORS enabled (the chapter 27 project does this).

**Docker (recommended, matches course docs):**

```bash
cd ../27-shop-api
docker compose up -d --build
```

After a fresh database, seed sample products from the host (Postgres is published on **5432** in `compose.yml`):

```bash
DATABASE_URL=postgres://user:secret@127.0.0.1:5432/mydb cargo run --bin seed
```

**Or** run the API locally: copy `27-shop-api/.env.example` → `.env`, start Postgres, then `cargo run --bin shop-api`.

See [`27-shop-api/README.md`](../27-shop-api/README.md) for every endpoint, troubleshooting, and port notes.

## 2. Point the UI at the API

The base URL is a constant in `src/main.rs`:

```rust
const API_BASE: &str = "http://127.0.0.1:8090";
```

- No trailing slash.
- Use the **same host** you use in the browser (`127.0.0.1` vs `localhost` are different origins for CORS). If you open the app at `http://localhost:8080`, either browse the API with `localhost` in `API_BASE` or open the UI at `http://127.0.0.1:8080`.

## 3. Run the shop UI

From **this directory** (`28-shop-ui`):

```bash
dx serve
```

By default the dev server listens on **`http://127.0.0.1:8080`** (check the `dx` output). Open that URL in a browser.

- **Catalog** loads `GET /api/products`.
- **Product** pages use `GET /api/products/{id}`.
- **Basket** uses `POST /api/baskets/{id}/items` and related routes; a basket id is stored in **local storage** (default id `demo` until you change it).

## Project layout

| Path | Purpose |
|------|--------|
| `src/main.rs` | Routes, layout, API `fetch` helpers, basket UI |
| `assets/shop.css` | Styles |
| `Cargo.toml` | `dioxus`, `dioxus-router`, `gloo-net`, `gloo-storage` |
| `Dioxus.toml` | Dioxus app / web settings |

## Other platforms

```bash
dx serve --platform desktop
```

(Requires the corresponding Dioxus feature / toolchain setup.)
