CREATE TABLE products (
    id INT PRIMARY KEY,
    title TEXT NOT NULL,
    price_cents INT NOT NULL CHECK (price_cents >= 0)
);

INSERT INTO products (id, title, price_cents)
VALUES
    (1, 'mug', 1000),
    (2, 'bowl', 2500),
    (3, 'teapot', 800);
