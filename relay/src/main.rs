use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use clap::Parser;
use nanoid::nanoid;
use shared::{
    Cache,
    consts::MAX_ROOMS,
    response::{CreateRoomResponse, RoomPrivacy},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use traccia::{LogLevel, fatal, info};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// The address that the server will listen on
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// The port that the server will listen on
    #[arg(short, long)]
    port: u16,
}

fn default_level() -> LogLevel {
    if cfg!(debug_assertions) {
        LogLevel::Debug
    } else {
        LogLevel::Info
    }
}

#[derive(Clone)]
struct AppState {
    /// Map to save room_id -> client data (String is a placeholder)
    rooms: Cache<String, Vec<String>>,
    addr: Arc<String>,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    traccia::init(default_level());

    let addr_str = format!("{}:{}", args.addr, args.port);
    let addr: SocketAddr = match addr_str.parse() {
        Ok(addr) => addr,
        Err(_) => {
            fatal!("Failed to parse address: {}", addr_str);
            return Ok(());
        }
    };

    let rooms = Cache::with_capacity(MAX_ROOMS);
    let state = AppState {
        rooms,
        addr: addr.to_string().into(),
    };

    let app = Router::new()
        .route("/room/create", post(create_room))
        .with_state(state);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            fatal!("Error while setting up listener: {}", e);
            return Ok(());
        }
    };

    info!("Relay listening on {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        fatal!("Error during serve:{}", e)
    }

    Ok(())
}

/// POST /room/create
///
/// Checks for available room space and creates it if it can.
/// Sends the room data back with self address for websocket connection with client
async fn create_room(State(state): State<AppState>) -> impl IntoResponse {
    if state.rooms.len().await == MAX_ROOMS {
        return StatusCode::INSUFFICIENT_STORAGE.into_response();
    }

    let room_id = nanoid!();

    info!("Creating room with {}", room_id);

    let payload = CreateRoomResponse {
        id: room_id,
        privacy: RoomPrivacy::Public,
        relay_addr: state.addr.to_string(),
    };

    (StatusCode::CREATED, Json(payload)).into_response()
}
