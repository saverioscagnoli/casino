use axum::{
    Router,
    routing::{get, post},
};
use clap::Parser;
use commands::{ClearCommand, HelpCommand, RelayCommand};
use console::{Command, Console};
use shared::ConcurrentHashMap;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;
use traccia::info;

mod commands;
mod log;
mod routes;

#[derive(Debug, Clone, Copy, Parser)]
#[clap(about = "This is the main hub for relay servers to connect to.")]
struct Args {
    /// The address to bind the server to
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: IpAddr,

    /// The port to bind the server to
    #[arg(short, long)]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    relays: ConcurrentHashMap<SocketAddr, reqwest::Client>,

    /// RoomID -> relay address
    /// Useful for relays to know where to send messages,
    /// like when a user joins a room.
    room_cache: ConcurrentHashMap<String, SocketAddr>,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    log::setup_logging();

    let state = AppState {
        relays: ConcurrentHashMap::new(),
        room_cache: ConcurrentHashMap::new(),
    };
    let app = Router::new()
        .route("/ping", get(routes::get::ping))
        .route("/session", post(routes::post::session))
        .route("/room/create", post(routes::post::create_room))
        .with_state(state.clone());

    let addr = SocketAddr::new(args.addr, args.port);
    let listener = TcpListener::bind(addr).await?;

    let server_handle = tokio::spawn(async move {
        info!("Starting server on {}", addr);
        axum::serve(listener, app).await
    });

    let relay_command = RelayCommand(state.relays.clone());

    let console_handle = tokio::spawn(async move {
        let commands = vec![(ClearCommand.name(), ClearCommand.description())];

        Console::new()
            .command(ClearCommand)
            .command(HelpCommand(commands))
            .command(relay_command)
            .case_sensitive(false)
            .prompt("> ")
            .run()
            .await
    });

    let _ = tokio::try_join!(server_handle, console_handle);

    Ok(())
}
