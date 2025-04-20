use axum::{
    Json, Router,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
};
use clap::Parser;
use commands::{ClearCommand, RelayCommand};
use console::Console;
use endpoints::Endpoint;
use reqwest::StatusCode;
use shared::{Cache, consts::MAX_RELAYS, response::CreateRoomResponse};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use traccia::{LogLevel, error, fatal, info};

mod commands;
mod endpoints;

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
    relays: Cache<String, reqwest::Client>,
    /// Maps the id of the room mapped to the id of the relay
    /// Used for quick join access
    room_cache: Cache<String, String>,
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

    let relays = Cache::with_capacity(MAX_RELAYS);
    let console = Console::new()
        .case_sensitive(false)
        .prompt("> ")
        .command(ClearCommand)
        .command(RelayCommand(relays.clone()));

    let console_handle = tokio::spawn(console.run());
    let state = AppState {
        relays,
        room_cache: Cache::new(),
    };

    let app = Router::new()
        .route("/greet/{name}", get(greet))
        .route("/room/create", post(create_room))
        .with_state(state);

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(e) => {
            fatal!("Error while setting up listener: {}", e);
            return Ok(());
        }
    };

    info!("Hub listening on {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        fatal!("Error during serve:{}", e)
    }

    _ = console_handle.await.unwrap();

    Ok(())
}

/// POST /room/create
/// Manages the internal relay map state
///
/// Checks for a relay with a room available and returns the first one
async fn create_room(State(state): State<AppState>) -> impl IntoResponse {
    let lock = state.relays.read().await;

    for (addr, client) in lock.iter() {
        let endpoint = Endpoint::CreateRoom(addr.to_string());

        let req = match client.post(endpoint.url()).build() {
            Ok(req) => req,
            Err(e) => {
                error!("Failed to build request: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        };

        let res = match client.execute(req).await {
            Ok(res) => res,
            Err(e) => {
                error!("{}", e);
                continue;
            }
        };

        if !res.status().is_success() {
            continue;
        }

        let body = match res.json::<CreateRoomResponse>().await {
            Ok(body) => body,
            Err(e) => {
                error!("Failed to deserialize body: {}", e);
                return StatusCode::BAD_REQUEST.into_response();
            }
        };

        info!(
            "Room created with id: {} on relay with address {}",
            body.id, addr
        );

        return (StatusCode::OK, Json(body)).into_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "There are no rooms available",
    )
        .into_response()
}

async fn greet(Path(name): Path<String>) -> String {
    info!("Greeting:{}", name);
    format!("Hello, {}!", name)
}
