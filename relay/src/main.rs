use axum::{
    Router,
    routing::{get, post},
};
use clap::Parser;
use shared::ConcurrentHashMap;
use std::net::{IpAddr, SocketAddr};
use tokio::net::TcpListener;

mod routes;

#[derive(Debug, Clone, Parser)]
#[clap(
    about = "This is the relay server. Clients will connect with this once the main server decudes."
)]
struct Args {
    /// The address to bind the server to
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: IpAddr,

    /// The port to bind the server to
    #[clap(short, long)]
    port: u16,
}

#[derive(Clone)]
struct AppState {
    /// Relay address -> array of socket clients
    rooms: ConcurrentHashMap<String, String>,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    let addr = SocketAddr::new(args.addr, args.port);
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    let state = AppState {
        rooms: ConcurrentHashMap::new(),
    };
    let app = Router::new()
        .route("/healthcheck", get(routes::get::healthcheck))
        .route("/room/create", post(routes::post::create_room))
        .with_state(state);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
