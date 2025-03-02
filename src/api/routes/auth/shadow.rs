//! Functionality for shadow authentication, impersonating another user
use crate::{
    api::{app_state::AppState, jwt::encode_jwt},
    db::{users::UserRole, User},
};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Extension, Json,
};
use reqwest::StatusCode;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ShadowUserParams {
    user_id: Uuid,
}

pub async fn shadow_user(
    State(state): State<Arc<AppState>>,
    Extension(current_user): Extension<Option<User>>,
    Json(params): Json<ShadowUserParams>,
) -> impl IntoResponse {
    // User must be both authenticated and have the role 'admin'
    let Some(user) = current_user else {
        return (StatusCode::UNAUTHORIZED, "Authentication required").into_response();
    };
    if user.role != UserRole::Admin {
        return (
            StatusCode::UNAUTHORIZED,
            "User does not have required permissions",
        )
            .into_response();
    }

    // Generate the user token for the target user
    let Ok(shadowed_user) = User::by_id(params.user_id, &state.db).await else {
        return (StatusCode::NOT_FOUND, "User not found").into_response();
    };

    let Ok(token) = encode_jwt(&shadowed_user.id.to_string()) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to encode JWT").into_response();
    };

    // Redirect to the frontend login page with the token
    let frontend_url = std::env::var("FRONTEND_URL").expect("Could not find frontend url");
    Redirect::to(&format!("{}/login?token={}", frontend_url, token)).into_response()
}
