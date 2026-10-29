//! Idempotent product catalog seed — run after you have a `products` table (see migrations).

use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(2)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    sqlx::query(
        r#"
        INSERT INTO products (id, title, price_cents, cost_cents) VALUES
          (1, 'Rust mug', 1299, 400),
          (2, 'Ferris sticker', 399, 120)
        ON CONFLICT (id) DO UPDATE SET
          title = EXCLUDED.title,
          price_cents = EXCLUDED.price_cents,
          cost_cents = EXCLUDED.cost_cents
        "#,
    )
    .execute(&pool)
    .await?;

    println!("Seed finished: products ready (ids 1–2).");
    Ok(())
}
