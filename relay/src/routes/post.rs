use crate::AppState;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use shared::{CreateRoom, RoomPrivacy};
use uuid::Uuid;

pub async fn create_room(State(state): State<AppState>) -> impl IntoResponse {
    let room_id = Uuid::new_v4();
    let body = CreateRoom {
        id: room_id.to_string(),
        privacy: RoomPrivacy::Public,
    };

    state.rooms.insert(room_id.to_string(), "".to_string());

    (StatusCode::OK, Json(body))
}
