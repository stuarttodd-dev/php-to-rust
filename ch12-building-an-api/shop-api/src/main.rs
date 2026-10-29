use std::net::SocketAddr;

struct Config {
    database_url: String,
    bind: String,
}

fn config() -> Config {
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL is not set");
            std::process::exit(1);
        }
    };
    let bind = std::env::var("BIND").unwrap_or_else(|_| String::from("127.0.0.1:8090"));
    Config { database_url, bind }
}

#[tokio::main]
async fn main() {
    let config = config();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let pool = sqlx::PgPool::connect(&config.database_url)
        .await
        .expect("connect to database");
    sqlx::migrate!().run(&pool).await.expect("run migrations");
    let listener = tokio::net::TcpListener::bind(&config.bind)
        .await
        .expect("bind listener");
    let addr = listener
        .local_addr()
        .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], 8090)));
    tracing::info!("listening on {addr}");
    axum::serve(listener, shop_api::app(pool))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("serve");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
