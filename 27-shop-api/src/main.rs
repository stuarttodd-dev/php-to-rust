//! Binary only: `.env`, `PgPool`, embedded migrations, `axum::serve`.

use shop_api::app;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    println!("Starting shop-api...");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(2))
        .connect(&database_url)
        .await?;

    println!("Database connection successful");
    println!("Running migrations...");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("Migrations completed");
    let pool = Arc::new(pool);

    println!("Server listening on 0.0.0.0:8080");
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app(pool)).await?;

    Ok(())
}
