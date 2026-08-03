use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::info;

pub async fn create_pool(database_url: &str) -> anyhow::Result<PgPool> {
    info!("Connecting to PostgreSQL database pool...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;

    info!("Database connection established successfully.");
    Ok(pool)
}
