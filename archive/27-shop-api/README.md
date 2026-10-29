# 27-shop-api (superseded)

The current Building an API chapter example is the Cargo workspace at
[`ch12-building-an-api`](../ch12-building-an-api/). That layout includes `shop`,
`discounts`, and `shop-api`, with a Dockerfile whose build context is the
workspace root so path dependencies work.

Use `ch12-building-an-api/shop-api` for Axum 0.8 `{id}` routes, `AppError` with
`Database(#[from] sqlx::Error)`, checkout `FOR UPDATE`, and `SQLX_OFFLINE` via
committed `.sqlx/`.
