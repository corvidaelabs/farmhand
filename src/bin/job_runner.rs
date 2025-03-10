use anyhow::Result;
use async_nats::jetstream::AckKind;
use farmhand::{
    event::MESSAGE_PREFIX,
    nats::create_nats_client,
    queue::{process_message, Queue},
};
use futures::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Connect to the stream
    tracing::debug!("Connecting to NATS server");
    let nats_client = create_nats_client().await?;
    tracing::debug!("Connecting to queue");
    let queue = Queue::connect(nats_client)
        .await
        .expect("Failed to create worker queue");

    // Get all jobs from the stream
    let subject = format!("{}.jobs.>", MESSAGE_PREFIX); // All farmhand jobs

    // TODO: Make this ID dynamic so we can run more than one runner at a time
    // Make sure not too make it too dynamic, as they are intended to be re-used
    let runner_name = "farmhand_runner_1".to_string();
    tracing::info!("Listening for jobs {} on {}", subject, runner_name);
    // Create the consumer to listen for jobs
    let consumer = queue.create_consumer(Some(runner_name), subject).await?;
    // Start consuming jobs
    loop {
        // TODO: Make this max_messages dynamic
        let mut jobs = consumer.fetch().max_messages(3).messages().await?;
        // Start processing jobs
        let mut handles = Vec::new();
        while let Some(job) = jobs.next().await {
            // Make sure the job is good to go
            let Ok(job) = job else {
                tracing::error!("Failed to receive job");
                continue;
            };
            // Process the message itself, ack on success, nack on failure
            let handle = tokio::spawn(async move {
                match process_message(&job.message).await {
                    Ok(_) => job.ack().await.expect("Failed to ack job"),
                    Err(err) => {
                        tracing::error!("Failed to process job: {}", err);
                        job.ack_with(AckKind::Nak(None))
                            .await
                            .expect("Failed to nack job");
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.expect("Task failed");
        }

        // Add a small delay to prevent tight loops when there are no jobs
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
