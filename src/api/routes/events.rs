use crate::{api::app_state::AppState, db::User, event::Event};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Extension, Json,
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct EventsQuery {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub username: String,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub events: Vec<Event>,
}

/// Gets all events for a given user and time range
pub async fn get_events(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<Option<User>>,
    stream_query: Query<EventsQuery>,
) -> impl IntoResponse {
    let Some(user) = user else {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    };

    if user.username != stream_query.username {
        return (StatusCode::FORBIDDEN, "Forbidden").into_response();
    }

    let Ok(target_user) = User::by_username(stream_query.username.clone(), &state.db).await else {
        return (StatusCode::NOT_FOUND, "User not found").into_response();
    };

    let Ok(events) = state
        .event_stream
        .get_user_events(
            target_user.username,
            stream_query.start_time,
            stream_query.end_time,
        )
        .await
    else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch events").into_response();
    };

    let response = EventResponse { events };

    (StatusCode::OK, Json(response)).into_response()
}
