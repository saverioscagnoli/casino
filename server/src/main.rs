use ::console::{CommandExecutor, Console, op::PrintLn};
use clap::Parser;
use console::{ClearCommand, RelayComand};
use http::{ApiHandler, App, Request, Response, async_trait};
use hyper::{Method, StatusCode};
use mini_moka::sync::Cache;
use nanoid::nanoid;
use payload::{LoginRequestBody, LoginResponseBody};
use std::{io::Write, net::SocketAddr, sync::LazyLock};
use traccia::{Hook, LogLevel, TargetId, error, fatal, info};

mod console;
mod payload;

struct ServerHandler;

pub static RELAYS: LazyLock<Cache<SocketAddr, String>> = LazyLock::new(|| Cache::new(100));

#[async_trait]
impl ApiHandler for ServerHandler {
    async fn incoming(&self, req: Request) -> Result<Response, hyper::Error> {
        let method = req.method();
        let segments = req.segments();

        match (method, &segments[..]) {
            (&Method::POST, ["session"]) => {
                let body = req.json::<LoginRequestBody>().await.unwrap();

                let id = nanoid!();
                let username = body.username;

                info!("New login: {}", username);

                let res = Response::empty()
                    .status(StatusCode::CREATED)
                    .body(LoginResponseBody { id, username })
                    .unwrap();

                Ok(res)
            }

            // Cors
            (&Method::OPTIONS, _) => Ok(Response::empty().status(StatusCode::OK)),

            _ => Ok(Response::empty().status(StatusCode::NOT_FOUND)),
        }
    }
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

fn setup_logger() {
    traccia::init_with_config(traccia::Config {
        level: default_level(),
        ..Default::default()
    });

    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            // Clear the current line and move the cursor to the beginning
            print!("\x1B[2K\x1B[0G");

            if let Err(e) = std::io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
        }
    })));

    traccia::set_hook(Hook::AfterLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            // Print the prompt again
            print!("> ");

            if let Err(e) = std::io::stdout().flush() {
                error!("Failed to flush stdout: {}", e);
            }
        }
    })));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    setup_logger();

    let args = Args::parse();

    let addr_str = format!("{}:{}", args.addr, args.port);

    let addr: SocketAddr = match addr_str.parse() {
        Ok(addr) => addr,
        Err(_) => {
            fatal!("Invalid address: {}", addr_str);
            return Ok(());
        }
    };

    _ = tokio::spawn(
        Console::new()
            .case_sensitive(false)
            .prompt("> ")
            .prompt_on_start(false)
            .command(ClearCommand)
            .command(RelayComand)
            .default_callback(|mut stdout, bad| async move {
                stdout
                    .execute(PrintLn(format!("Unknown command '{}'.", bad)))
                    .await
            })
            .run(),
    );

    if let Ok(app) = App::new(addr).await {
        info!("Server listening on {}", addr);

        if let Err(e) = app.run(ServerHandler).await {
            fatal!("There was an error during the main app loop: {}", e);
        }
    }

    Ok(())
}
