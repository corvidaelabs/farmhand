use super::Event;
use crate::{
    error::StreamError,
    event::{EVENT_PREFIX, MESSAGE_PREFIX},
    twitch::ChatMessagePayload,
};
use async_nats::{
    jetstream::{
        self,
        consumer::{pull::Config, Consumer, DeliverPolicy},
        Context,
    },
    Client,
};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use std::time::Duration;
use time::OffsetDateTime;

pub const EVENT_STREAM: &str = "FARMHAND_EVENTS";

pub struct Stream {
    name: String,
    jetstream: Context,
}

impl Stream {
    /// Connects to an existing queue
    pub async fn connect(nats_client: Client) -> Result<Self, StreamError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .get_stream(EVENT_STREAM)
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        Ok(Stream {
            name: EVENT_STREAM.to_string(),
            jetstream,
        })
    }
    /// Creates a new queue
    pub async fn new(
        name: String,
        description: Option<String>,
        subjects: Vec<String>,
        nats_client: Client,
    ) -> Result<Self, StreamError> {
        let jetstream = Self::create_jetstream(nats_client);
        jetstream
            .create_stream(jetstream::stream::Config {
                name: name.clone(),
                subjects,
                description,
                // max_bytes: 1024 * 1024 * 1024, // 1GB
                max_age: Duration::from_secs(60 * 60 * 24 * 30), // 1 month
                ..Default::default()
            })
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        Ok(Stream { name, jetstream })
    }
    /// Deletes the stream
    pub async fn delete(nats_client: Client) -> Result<(), StreamError> {
        let jetstream = Self::create_jetstream(nats_client);

        // Check if stream exists first
        if jetstream.get_stream(EVENT_STREAM).await.is_ok() {
            jetstream
                .delete_stream(EVENT_STREAM)
                .await
                .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;
        } else {
            tracing::warn!("Stream {} does not exist", EVENT_STREAM);
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
    ) -> Result<Consumer<Config>, StreamError> {
        let config = jetstream::consumer::pull::Config {
            durable_name: name,
            filter_subject: filter,
            max_deliver: 3,
            ..Default::default()
        };
        self.jetstream
            .create_consumer_on_stream(config, self.name.to_string())
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))
    }
    /// Publishes a message to the queue
    pub async fn publish(&self, subject: String, message: String) -> Result<(), StreamError> {
        tracing::debug!("Publishing message to subject {}", subject);
        self.jetstream
            .publish(subject, message.into())
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;

        Ok(())
    }
    /// Gets the subject for all events by a user
    fn get_subject_all_user_events(&self, username: String) -> String {
        format!(
            "{}.{}.twitch.events.{}.>",
            MESSAGE_PREFIX, EVENT_PREFIX, username
        )
    }
    /// Gets all events within time range for subject
    pub async fn get_user_events(
        &self,
        username: String,
        start_time: DateTime<Utc>,
        end_time: Option<DateTime<Utc>>,
    ) -> Result<Vec<Event>, StreamError> {
        // Create the subject for the given user
        let subject = self.get_subject_all_user_events(username);
        // Configure a consumer to get all events after the start time
        let consumer_config = jetstream::consumer::pull::Config {
            filter_subject: subject,
            max_deliver: 3,
            deliver_policy: DeliverPolicy::ByStartTime {
                start_time: {
                    let timestamp = start_time.timestamp();
                    let nanoseconds = start_time.timestamp_subsec_nanos();

                    OffsetDateTime::from_unix_timestamp(timestamp)?
                        .replace_nanosecond(nanoseconds as u32)?
                },
            },
            ack_policy: jetstream::consumer::AckPolicy::None,
            ..Default::default()
        };

        // Create the consumer
        let consumer = self
            .jetstream
            .create_consumer_on_stream(consumer_config, self.name.to_string())
            .await
            .map_err(|e| StreamError::InvalidConnection(e.to_string()))?;

        // Initialize the events vector
        let mut events = Vec::new();
        let mut batch = consumer.fetch().max_messages(10000).messages().await?;
        while let Some(message) = batch.next().await {
            let Ok(message) = message else {
                tracing::error!("Failed to unwrap message: {:?}", message);
                continue;
            };
            let timestamp_utc: DateTime<Utc> = match message.info() {
                Ok(info) => {
                    // Attempt to parse the timestamp
                    let timestamp = DateTime::from_timestamp(
                        info.published.unix_timestamp(),
                        info.published.nanosecond(),
                    );

                    // If the timestamp is invalid, log an error and continue
                    match timestamp {
                        Some(timestamp) => timestamp,
                        None => {
                            tracing::error!("Failed to parse timestamp");
                            continue;
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to get message info: {:?}", e);
                    continue;
                }
            };

            if let Some(end_time) = end_time {
                tracing::debug!(
                    "Checking for end time against {}, message timestamp: {}",
                    end_time,
                    timestamp_utc
                );
                // Check if the message is after the end time
                if timestamp_utc > end_time {
                    tracing::debug!("Message is after end time, total events: {}", events.len());
                    break;
                }
            }

            // Parse the event
            let event = serde_json::from_slice::<Event>(&message.payload);
            match event {
                Ok(event) => events.push(event),
                Err(e) => {
                    // Some old chat messages are sent without the event wrapper, so attempt to parse them as a raw message
                    // TODO: Remove in the next major release
                    match serde_json::from_slice::<ChatMessagePayload>(&message.payload) {
                        Ok(raw_message) => events.push(Event::from(raw_message)),
                        Err(_) => tracing::error!("Failed to parse event: {:?}", e),
                    }
                }
            };

            tracing::debug!("Events size: {}", events.len());
        }
        Ok(events)
    }
}
