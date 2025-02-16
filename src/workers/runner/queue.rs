use async_nats::{
    jetstream::{
        self,
        consumer::{pull::Config, Consumer},
        Context,
    },
    Client,
};

use crate::error::QueueError;

#[allow(dead_code)]
/// TODO: Remove dead code annotation after implementing
pub struct Queue {
    name: String,
    jetstream: Context,
}

impl Queue {
    /// Connects to an existing queue
    pub async fn connect(name: String, nats_client: Client) -> Result<Self, QueueError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .get_stream(&name)
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        Ok(Queue { name, jetstream })
    }
    /// Creates a new queue
    pub async fn new(
        name: String,
        description: Option<String>,
        subjects: Vec<String>,
        nats_client: Client,
    ) -> Result<Self, QueueError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .create_stream(jetstream::stream::Config {
                name: name.clone(),
                subjects,
                description,
                ..Default::default()
            })
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        Ok(Queue { name, jetstream })
    }
    /// Deletes the queue
    pub async fn delete(name: String, nats_client: Client) -> Result<(), QueueError> {
        let jetstream = Self::create_jetstream(nats_client);

        // Check if stream exists first
        if jetstream.get_stream(&name).await.is_ok() {
            jetstream
                .delete_stream(&name)
                .await
                .map_err(|e| QueueError::InvalidConnection(e.to_string()))?;
        } else {
            tracing::warn!("Stream {} does not exist", name);
        }
        Ok(())
    }
    /// Creates a new jetstream context
    fn create_jetstream(nats_client: Client) -> Context {
        jetstream::new(nats_client)
    }
    /// Creates a new consumer
    pub async fn create_consumer(
        &self,
        name: Option<String>,
        filter: String,
    ) -> Result<Consumer<Config>, QueueError> {
        let config = jetstream::consumer::pull::Config {
            durable_name: name,
            filter_subject: filter,
            ..Default::default()
        };
        self.jetstream
            .create_consumer_on_stream(config, self.name.to_string())
            .await
            .map_err(|e| QueueError::InvalidConnection(e.to_string()))
    }
}
