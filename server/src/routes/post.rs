use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use shared::CreateRoom;
use traccia::info;
use uuid::Uuid;

use crate::AppState;

pub async fn session(State(state): State<AppState>) -> impl IntoResponse {
    StatusCode::OK
}

pub async fn create_room(State(state): State<AppState>) -> impl IntoResponse {
    for relay in state.relays.iter() {
        let (addr, client) = relay.pair();

        let url = format!("http://{}/room/create", addr);

        match client.post(&url).send().await {
            Ok(response) => {
                if response.status() == StatusCode::OK {
                    match response.json::<CreateRoom>().await {
                        Ok(room) => {
                            info!("Created room: {}", room.id);
                            state.room_cache.insert(room.id, *addr);
                        }

                        Err(_) => {}
                    }
                }
            }

            Err(_) => {}
        }
    }
}
