use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("⚙️ DevResume AI Background Worker starting...");

    loop {
        info!("Checking queue for repository sync and AI resume generation jobs...");
        sleep(Duration::from_secs(10)).await;
    }
}
