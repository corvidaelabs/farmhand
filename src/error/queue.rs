use async_nats::jetstream::consumer::pull::BatchErrorKind;
use axum::{http, response::IntoResponse};
use http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Invalid Connection: {0}")]
    InvalidConnection(String),
}

#[derive(Error, Debug)]
pub enum StreamError {
    #[error("Error parsing event time offset")]
    ParseTimeOffset(#[from] time::error::ComponentRange),
    #[error("Error parsing event")]
    ParseEvent(#[from] serde_json::Error),
    #[error("NATS stream batch error")]
    Batch(#[from] async_nats::error::Error<BatchErrorKind>),
    #[error("Invalid Connection: {0}")]
    InvalidConnection(String),
}

impl IntoResponse for StreamError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
