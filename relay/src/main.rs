use std::net::SocketAddr;

use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use clap::Parser;
use mini_moka::sync::Cache;
use nanoid::nanoid;
use serde::Serialize;
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

type ClientMap = Cache<String, ()>;

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

    let clients: ClientMap = Cache::new(10_000);
    let app = Router::new()
        .route("/room/create", post(create_room))
        .with_state(clients);

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

#[derive(Serialize)]
struct CreateRoomResponse {
    id: String,
}

async fn create_room(State(clients): State<ClientMap>) -> impl IntoResponse {
    let id = nanoid!();

    info!("Creating room with {}", id);

    let payload = CreateRoomResponse { id };
    (StatusCode::CREATED, Json(payload))
}
