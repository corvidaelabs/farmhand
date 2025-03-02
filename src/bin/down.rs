use anyhow::Result;
use farmhand::{db, event::Stream, nats::create_nats_client, queue::Queue};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "down=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::warn!("Deleting all data from the project, this is a destructive operation");
    for i in (1..=5).rev() {
        tracing::warn!("Deleting all data in {} seconds...", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    tracing::info!("Starting deletion process");
    let _ = tokio::join!(down_db(), down_nats());
    tracing::info!("Successfully deleted all data from the project");
    Ok(())
}

async fn down_db() -> Result<()> {
    // Connect to the database
    let db_pool = db::connect_to_database().await?;

    // Delete all data from the database
    tracing::debug!("Deleting all data from the database");
    db::delete_all_data(&db_pool).await?;
    tracing::info!("Successfully deleted all data from the database");
    Ok(())
}

async fn down_nats() -> Result<()> {
    // Delete all streams
    tracing::debug!("Deleting all streams");

    // Create NATS client
    let nats_client = create_nats_client().await?;

    // Delete the job queue
    tracing::debug!("Deleting job queue");
    Queue::delete(nats_client.clone()).await?;
    // Delete the event stream
    tracing::debug!("Deleting event stream");
    Stream::delete(nats_client.clone()).await?;

    tracing::info!("Successfully deleted all streams");
    Ok(())
}
