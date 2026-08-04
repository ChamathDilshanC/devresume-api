mod handlers;
mod routes;
mod state;

use common::db;
use common::Config;
use state::AppState;
use std::net::SocketAddr;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();

    // Connect to Database
    let pool = db::create_pool(&config.database_url).await?;

    info!("Running database migrations...");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to run migrations: {}", e);
            e
        })?;
    info!("Database migrations applied successfully.");

    let state = AppState {
        db: pool,
        config: config.clone(),
    };

    let app = routes::create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    info!("🚀 DevResume AI Axum Backend listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
