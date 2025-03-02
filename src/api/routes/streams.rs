use crate::{
    api::app_state::AppState,
    db::{streams::Stream, User},
};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Serialize)]
struct StreamResponse {
    streams: Vec<Stream>,
}

#[derive(Deserialize)]
pub struct StreamQuery {
    stream_id: Uuid,
}

/// Gets a list of streams by user or stream ID
pub async fn get_streams(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    stream_query: Option<Query<StreamQuery>>,
) -> impl IntoResponse {
    let Some(user) = user else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "User not found").into_response();
    };
    // If there's a stream_id in the query, find the stream by ID
    if let Some(query) = stream_query {
        let stream_id = query.0.stream_id;
        let Ok(stream) = Stream::find_by_id(stream_id, &state.db).await else {
            return (StatusCode::NOT_FOUND, "Stream not found").into_response();
        };

        // Make sure the user owns the stream
        if stream.user_id != user.id {
            return (StatusCode::FORBIDDEN, "You do not own this stream").into_response();
        }

        return (
            StatusCode::OK,
            Json(StreamResponse {
                streams: vec![stream],
            }),
        )
            .into_response();
    }

    // Rest of the function implementation
    let streams = match Stream::find_by_user_id(user.id, &state.db).await {
        Ok(streams) => streams,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    (StatusCode::OK, Json(StreamResponse { streams })).into_response()
}
