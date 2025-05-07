use axum::{http::StatusCode, response::IntoResponse};

/// Health check, used when the server wants to add this relay to its list of relays
/// and check if it is alive.
pub async fn healthcheck() -> impl IntoResponse {
    StatusCode::OK
}
