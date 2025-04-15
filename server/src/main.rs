use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use ::console::op::PrintLn;
use ::console::{CommandExecutor, Console};
use bytes::Bytes;
use clap::Parser;
use console::{AddRelayComand, ClearCommand, ListRelaysCommand};
use http_body_util::{BodyExt, Empty, Full, combinators::BoxBody};
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response, StatusCode, body::Body};
use hyper_util::rt::TokioIo;
use mini_moka::sync::Cache;
use nanoid::nanoid;
use tokio::net::TcpListener;
use traccia::{Colorize, Hook, LogLevel, TargetId, error, info};

mod console;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let method = req.method();
    let path = req.uri().path();
    let segments = path
        .split('/')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();
    let slice = segments.as_slice();

    match (method, slice) {
        (&Method::GET, ["create_room"]) => {
            // Send a request to the relay servers
            // And create a room on one of them
            let room_id = nanoid!();

            info!("Creating room with id: {}", room_id);

            Ok(Response::new(full(room_id)))
        }

        (&Method::GET, ["room", id]) => {
            info!("Fetching room with id: {}", id);

            Ok(Response::new(full(format!("Room ID: {}", id))))
        }

        _ => {
            let mut not_found = Response::new(empty());

            *not_found.status_mut() = StatusCode::NOT_FOUND;

            Ok(not_found)
        }
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[derive(Debug, Parser)]
#[command(about, author, version)]
struct Args {
    /// The address the server will use
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();

    traccia::init_with_config(traccia::Config {
        level: default_level(),
        ..Default::default()
    });

    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            print!("\x1B[2K\x1B[G");
        }
    })));

    traccia::set_hook(Hook::AfterLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            print!("> ");
            std::io::stdout().flush().unwrap();
        }
    })));

    let ip = IpAddr::from_str(&args.addr).expect("Failed to parse id address");
    let addr = SocketAddr::new(ip, args.port);

    let listener = TcpListener::bind(addr).await?;

    let relays = Cache::new(120);

    let console = Console::new()
        .prompt("> ")
        .prompt_on_start(false)
        .case_sensitive(false)
        .command(ClearCommand)
        .command(AddRelayComand(relays.clone()))
        .command(ListRelaysCommand(relays.clone()))
        .default_callback(|mut stdout, bad| async move {
            stdout
                .execute(PrintLn(format!("Unknown command '{}'", bad)))
                .await?;

            Ok(())
        });

    let console_handle = tokio::spawn(console.run());

    info!(
        "Listening on {}",
        format!("http://{}", addr).color(traccia::Color::Cyan)
    );

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handler))
                .await
            {
                error!("Error serving connection: {:?}", err);
            }
        });
    }

    console_handle.await;
}
