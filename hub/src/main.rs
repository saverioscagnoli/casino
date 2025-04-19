use axum::{
    Router,
    extract::{Path, State},
    routing::{get, post},
};
use clap::Parser;
use commands::{ClearCommand, RelayCommand};
use console::Console;
use mini_moka::sync::Cache;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use traccia::{LogLevel, fatal, info};

mod commands;

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

pub type ClientMap = Cache<String, reqwest::Client>;

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

    let relays: ClientMap = Cache::new(100);
    let console = Console::new()
        .case_sensitive(false)
        .prompt("> ")
        .command(ClearCommand)
        .command(RelayCommand(relays.clone()));

    let console_handle = tokio::spawn(console.run());

    let app = Router::new()
        .route("/greet/{name}", get(greet))
        .route("/room/create", post(create_room))
        .with_state(relays);

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

#[derive(Deserialize)]
struct CreateRoomResponse {
    id: String,
}

async fn create_room(State(relays): State<ClientMap>) {
    for entry in relays.iter() {
        let addr = entry.key();
        let client = entry.value();

        let req = client.post(format!("http://{}/room/create", addr)).build().unwrap();
        let res = client.execute(req).await.unwrap();

        let body: CreateRoomResponse = res.json().await.unwrap();

        info!(
            "Room created with id: {} on relay with address {}",
            body.id, addr
        );
    }
}

async fn greet(Path(name): Path<String>) -> String {
    info!("Greeting:{}", name);
    format!("Hello, {}!", name)
}
