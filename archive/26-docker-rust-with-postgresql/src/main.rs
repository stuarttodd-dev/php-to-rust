use axum::{
    response::Json,
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL not set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
        .expect("Failed to ping database");

    log::info!("Database connection successful");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind port");

    log::info!("Server listening on 0.0.0.0:8080");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "OK",
        "service": "docker-rust-with-postgresql"
    }))
}

async fn root() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Rust API with PostgreSQL",
        "endpoints": [
            "/health - Health check",
            "/ - This message"
        ]
    }))
}
