# Chapter 12: Building an API

Workspace for the [Building an API](https://www.halfshellstudios.com/learn) chapter: `shop`, `discounts`, and `shop-api`.

```bash
export DATABASE_URL=postgres://shop:shop@127.0.0.1:54329/shop
cd shop-api
cargo sqlx migrate run
cargo sqlx prepare
cargo test --test api
```

Docker (from `shop-api/`):

```bash
docker compose up --build -d
curl -s http://127.0.0.1:8090/health
```

The compose build context is the workspace root so path dependencies (`../shop`, `../discounts`) are inside the context.
