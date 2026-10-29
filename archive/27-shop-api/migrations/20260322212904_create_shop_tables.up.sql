-- Add up migration script here
-- Catalog (maps to domain Product)
CREATE TABLE products (
    id          BIGSERIAL PRIMARY KEY,
    title       TEXT NOT NULL,
    price_cents INTEGER NOT NULL CHECK (price_cents > 0)
);

-- Basket lines keyed by basket id (maps to BasketLine + URL /api/baskets/:basket_id)
CREATE TABLE basket_lines (
    id          BIGSERIAL PRIMARY KEY,
    basket_id     TEXT NOT NULL,
    product_id  BIGINT NOT NULL REFERENCES products (id) ON DELETE CASCADE,
    qty         INTEGER NOT NULL CHECK (qty > 0),
    UNIQUE (basket_id, product_id)
);

CREATE INDEX basket_lines_basket_idx ON basket_lines (basket_id);
