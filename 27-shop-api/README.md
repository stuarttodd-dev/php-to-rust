# 27-shop-api

![php-to-rust logo](../images/php-to-rust.png)

Shop API: Axum + SQLx + PostgreSQL (catalog products and basket lines). Use Docker Compose for the database and app, or run the binary locally against a Postgres instance.

## Prerequisites

- Docker Desktop (or Docker Engine + Compose plugin), **or** a local PostgreSQL 15+ and Rust toolchain
- `curl` (examples below use `http://localhost:8080`)
- Ports **8080** (API) and **5432** (Postgres, if published) available

## Project layout

| Path | Purpose |
|------|---------|
| `Dockerfile` | Multi-stage build; runtime image runs `./server` (package binary `shop-api`) |
| `compose.yml` | `app` + `db` services, `DATABASE_URL`, healthcheck, `pgdata` volume |
| `migrations/` | SQLx embedded migrations (products, `cost_cents`, basket lines) |
| `src/main.rs` | Loads `.env`, connects, runs migrations, serves on `0.0.0.0:8080` |
| `src/bin/seed.rs` | Idempotent catalog seed (products 1–2) |
| `src/api/` | Routes, request DTOs, `ProductResponse`, validation |
| `src/domain/` | Domain types (`Product`, `Basket`, …) |
| `tests/` | Integration tests (`DATABASE_URL` must reach Postgres) |
| `.env.example` | Template for **host** runs and `cargo test` (`127.0.0.1`, not hostname `db`) |

## Run with Docker Compose

From this directory (`27-shop-api`):

```bash
cd 27-shop-api
docker compose up -d --build
```

This builds the API image, starts Postgres, waits for `db` to be healthy, then starts `app` on port **8080**. Data persists in the `pgdata` volume until you run `docker compose down -v`.

After the first start (or an empty database), load sample products from your **host** (port `5432` is published):

```bash
DATABASE_URL=postgres://user:secret@127.0.0.1:5432/mydb cargo run --bin seed
```

### Rebuild after changing Rust code

The container serves whatever binary was in the **image** when it was built. After editing sources:

```bash
docker compose build app && docker compose up -d app
```

### Stop the stack

```bash
docker compose down
```

Remove containers **and** wipe the database volume:

```bash
docker compose down -v
```

### Logs and status

```bash
docker compose ps
docker compose logs -f app
docker compose logs db
```

## Run locally (without Docker for the app)

1. Start Postgres (Compose db only is fine): `docker compose up -d db`
2. Copy env: `cp .env.example .env` and ensure `DATABASE_URL` uses **`127.0.0.1`** (not `db`).
3. `cargo run --bin shop-api`

Migrations run on startup; then seed if needed: `cargo run --bin seed`.

## API base URL

Use **`http://localhost:8080`** (or `http://127.0.0.1:8080`) when the app is bound to 8080.

---

## Endpoints

Success bodies use JSON unless noted. Errors are typically `{"error":"…"}`; validation errors for basket `qty` may include `"field":"qty"`.

### `GET /`

Service metadata and a short list of routes.

```bash
curl -s http://localhost:8080/
```

### `GET /api/health`

Liveness: pings the database.

```bash
curl -s http://localhost:8080/api/health
```

### `GET /api/products`

Catalog as a JSON array of **public** product objects (`id`, `title`, `price_cents` only — no internal fields such as `cost_cents`).

```bash
curl -s http://localhost:8080/api/products
```

### `GET /api/products/{id}`

Single product in the same **public** shape. `404` if missing.

```bash
curl -s http://localhost:8080/api/products/1
```

### `POST /api/baskets/{basket_id}/items`

Add or merge a line (`ON CONFLICT` adds quantities). Body must be JSON:

```json
{"product_id": 1, "qty": 2}
```

- **`204 No Content`** on success  
- **`400`** with `field: "qty"` if `qty` is 0  
- **`404`** `{"error":"product not found"}` if `product_id` does not exist  
- Invalid JSON syntax / shape: client error (e.g. **`400`** / **`422`** depending on Axum)

```bash
curl -s -i -X POST http://localhost:8080/api/baskets/demo/items \
  -H "Content-Type: application/json" \
  -d '{"product_id":1,"qty":2}'
```

Use the **same** `basket_id` when reading the basket below.

### `GET /api/baskets/{basket_id}`

Basket lines with titles, per-line subtotals, and `subtotal_cents`.

```bash
curl -s http://localhost:8080/api/baskets/demo
```

### `PATCH /api/baskets/{basket_id}/items/{product_id}`

Set quantity for one line. Body: `{"qty": <positive integer>}`.

- **`204`** on success  
- **`400`** if `qty` is 0 (same `field` pattern as POST)  
- **`404`** if the line does not exist  

```bash
curl -s -i -X PATCH http://localhost:8080/api/baskets/demo/items/1 \
  -H "Content-Type: application/json" \
  -d '{"qty":3}'
```

### `DELETE /api/baskets/{basket_id}/items/{product_id}`

Remove one line. **`204`** if a row was deleted, **`404`** if not found.

```bash
curl -s -i -X DELETE http://localhost:8080/api/baskets/demo/items/1
```

### `DELETE /api/baskets/{basket_id}`

Remove all lines for that basket. **`204`** on success.

```bash
curl -s -i -X DELETE http://localhost:8080/api/baskets/demo
```

---

## Configuration (Compose)

| Setting | Value |
|---------|--------|
| Postgres user / password / DB | `user` / `secret` / `mydb` |
| App `DATABASE_URL` inside Compose | `postgres://user:secret@db:5432/mydb` |
| API port | `8080` → container `8080` |
| Postgres port | `5432` → host `5432` (for host tools and `cargo test` / `seed`) |

### psql via Docker

```bash
docker compose exec db psql -U user -d mydb
```

### psql from the host

```bash
PGPASSWORD=secret psql -h 127.0.0.1 -p 5432 -U user -d mydb
```

## Tests

Integration tests need Postgres reachable at `DATABASE_URL` (see `.env.example`). Example:

```bash
docker compose up -d db
export DATABASE_URL=postgres://user:secret@127.0.0.1:5432/mydb
cargo test
```

## Troubleshooting

| Issue | What to try |
|-------|-------------|
| Port 8080 in use | Change mapping in `compose.yml` (e.g. `"8081:8080"`) or stop the other process. |
| `404` on `/api/baskets/...` but `/api/health` works | Rebuild and recreate the **app** container so it runs a binary that includes basket routes. |
| `PoolTimedOut` / tests cannot connect | Start `db`, use **`127.0.0.1`** in `DATABASE_URL` on the host (not hostname `db`). |
| Empty catalog | Run `cargo run --bin seed` against the same database the app uses. |

## Compose cheat sheet

```bash
docker compose up -d --build    # build + start
docker compose build app && docker compose up -d app   # rebuild API only
docker compose ps
docker compose logs -f app
docker compose down             # stop; keep volume
docker compose down -v        # stop + delete pgdata
docker compose build --no-cache app
```

## Key concepts

- **Migrations** run automatically when `shop-api` starts (`sqlx::migrate!` in `main.rs`).
- **Public JSON** for products uses **`ProductResponse`** so internal columns (e.g. `cost_cents`) never appear on the wire.
- **Baskets** are keyed by arbitrary string IDs (e.g. session or client-provided); lines live in `basket_lines`.
