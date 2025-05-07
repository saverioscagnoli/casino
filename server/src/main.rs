use axum::{Router, routing::get};
use clap::Parser;
use commands::{ClearCommand, HelpCommand};
use console::{Command, Console};
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

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let args = Args::parse();

    log::setup_logging();

    let app = Router::new().route("/ping", get(routes::get::ping));
    let addr = SocketAddr::new(args.addr, args.port);
    let listener = TcpListener::bind(addr).await?;

    let server_handle = tokio::spawn(async move {
        info!("Starting server on {}", addr);
        axum::serve(listener, app).await
    });

    let commands = vec![(ClearCommand.name(), ClearCommand.description())];

    let console_handle = tokio::spawn(async move {
        Console::new()
            .command(ClearCommand)
            .command(HelpCommand(commands))
            .case_sensitive(false)
            .prompt("> ")
            .run()
            .await
    });

    let _ = tokio::try_join!(server_handle, console_handle);

    Ok(())
}
