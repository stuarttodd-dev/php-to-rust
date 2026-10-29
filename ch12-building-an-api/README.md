# Chapter 12: Building an API

Workspace for the Building an API chapter: `shop`, `discounts`, and `shop-api`.

```bash
export DATABASE_URL=postgres://shop:shop@127.0.0.1:54329/shop
cd ch12-building-an-api
cargo test --workspace
# integration tests that need Postgres:
cd shop-api && cargo sqlx migrate run && cargo test --test api
```

Docker (from `shop-api/`):

```bash
docker compose up --build -d
curl -s http://127.0.0.1:8090/health
```

The compose build context is this workspace folder so path dependencies (`../shop`, `../discounts`) are inside the context.

A separate `reviews` crate beside this workspace covers the Laravel-style capstone lesson (`cargo test --manifest-path reviews/Cargo.toml`).
