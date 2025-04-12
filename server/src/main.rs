use clap::Parser;
use console::console_task;
use futures_util::{
    StreamExt,
    future::{self, Either},
};
use json::{TextPayload, broadcast};
use nanoid::nanoid;
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{Mutex, broadcast},
};
use tokio_tungstenite::{accept_async, tungstenite::Message};
use traccia::{Colorize, Hook, LogLevel, Style, TargetId, error, info, warn};
use ws::Ws;

mod console;
mod error;
mod json;
mod ws;

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        let gray = traccia::Color::RGB(128, 128, 128);

        match record.level {
            LogLevel::Error | LogLevel::Fatal => {
                format!(
                    "[{} {} @{} {}] {}",
                    chrono::Local::now()
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                        .color(gray),
                    record.level.default_coloring().to_lowercase(),
                    format!("{}:{}", record.file.unwrap(), record.line.unwrap()),
                    record.target.color(gray).italic(),
                    record.message
                )
            }

            level => {
                format!(
                    "[{} {} {}] {}",
                    chrono::Local::now()
                        .format("%Y-%m-%d %H:%M")
                        .to_string()
                        .color(gray),
                    level.default_coloring().to_lowercase(),
                    record.target.color(gray).italic(),
                    record.message
                )
            }
        }
    }
}

fn default_level() -> LogLevel {
    #[cfg(debug_assertions)]
    {
        LogLevel::Debug
    }

    #[cfg(not(debug_assertions))]
    {
        LogLevel::Info
    }
}

#[derive(Parser)]
struct Args {
    /// The address of the websocket server
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,
    /// The port to listen on
    #[arg(short, long)]
    port: u16,
    /// the log level to use
    #[arg(short, long, default_value_t = default_level())]
    level: LogLevel,
}

#[tokio::main]
async fn main() {
    console::clear();
    console::print_prompt();

    let args = Args::parse();

    traccia::set_hook(Hook::BeforeLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            // Clear the line before printing a log
            console::clear_line();
        }
    })));

    traccia::set_hook(Hook::AfterLog(Box::new(|_, target| {
        if let TargetId::Console(_) = target {
            // Print a new line after a log appears
            // After any log, restore the prompt
            console::print_prompt();
        }
    })));

    traccia::init_with_config(traccia::Config {
        level: args.level,
        format: Some(Box::new(CustomFormatter)),
        ..Default::default()
    });

    let addr: SocketAddr = format!("{}:{}", args.addr, args.port)
        .parse()
        .expect("Failed to parse address");

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Can't bind to address");

    info!("Websocket listening on {}", addr);
    info!("Type 'help' for available commands.");

    // Create a shutdown signal
    let (shutdown_tx, _) = broadcast::channel::<()>(1);

    let console_task = tokio::spawn(console_task(shutdown_tx.clone()));
    let server_task = tokio::spawn(server_task(listener, shutdown_tx.clone()));

    // Wait for server task to complete (after shutdown signal)
    let _ = server_task.await;
    let _ = console_task.await;

    info!("Server shutdown complete.");
}

// Externalized server handler function
async fn server_task(listener: TcpListener, shutdown_tx: broadcast::Sender<()>) {
    let mut shutdown_rx = shutdown_tx.subscribe();

    loop {
        let accept = listener.accept();
        let shutdown = shutdown_rx.recv();

        tokio::pin!(accept);
        tokio::pin!(shutdown);

        match future::select(accept, shutdown).await {
            Either::Left((result, _)) => match result {
                Ok((stream, addr)) => {
                    let shutdown_rx = shutdown_tx.subscribe();

                    tokio::spawn(connection_task(stream, addr, shutdown_rx));
                }

                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            },

            _ => {
                warn!("Server shutting down...");
                break;
            }
        }
    }
}

async fn connection_task(
    stream: tokio::net::TcpStream,
    addr: SocketAddr,
    shutdown_rx: broadcast::Receiver<()>,
) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during WebSocket handshake: {}", e);
            return;
        }
    };

    let (write, mut read) = ws_stream.split();
    let ws = Ws(Arc::new(Mutex::new(write)));
    let id = nanoid!();

    ws::insert_client(id.clone(), ws.clone());

    // Send hello payload to client if it is a new connection
    if let Err(e) = ws.send_hello(id.clone()).await {
        error!("{}", e);
        return;
    }

    info!("Client connected from: {}; assigned id: {}", addr, id);

    // Subscribe to broadcast channel and handle broadcast messages in a separate task
    let broadcast_task = broadcast_handler(id.clone(), ws.clone(), shutdown_rx);

    // Handle incoming client messages
    handle_client_messages(&mut read, &id).await;

    // Cleanup when client disconnects
    ws::remove_client(&id);
    broadcast_task.abort();
    warn!("Connection closed: {}", addr);
}

// Spawns a task to handle broadcast messages
fn broadcast_handler(
    id: String,
    ws: Ws,
    mut shutdown_rx: broadcast::Receiver<()>,
) -> tokio::task::JoinHandle<()> {
    // Subscribe to broadcast channel
    let mut broadcast_rx = broadcast().subscribe();

    tokio::spawn(async move {
        loop {
            // Use futures-util select instead of tokio::select! macro
            let broadcast = broadcast_rx.recv();
            let shutdown = shutdown_rx.recv();

            tokio::pin!(broadcast);
            tokio::pin!(shutdown);

            match future::select(broadcast, shutdown).await {
                Either::Left((result, _)) => {
                    if let Ok(payload) = result {
                        // Only forward messages that aren't from this client
                        if let TextPayload::ChatMessage(ref msg) = payload {
                            if msg.author_id != id {
                                if let Err(e) = ws.send_json(&payload).await {
                                    error!("{}", e);
                                    break;
                                }
                            }
                        }
                    }
                }

                _ => {
                    // Server is shutting down
                    break;
                }
            }
        }
    })
}

// Handle client messages without tokio::select!
async fn handle_client_messages(
    read: &mut futures_util::stream::SplitStream<
        tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    >,
    _id: &str,
) {
    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Binary(bytes) => {
                info!("received: {}", String::from_utf8(bytes.to_vec()).unwrap())
            }

            Message::Text(raw) => match serde_json::from_str::<TextPayload>(&raw) {
                Ok(payload) => match payload {
                    TextPayload::ChatMessage(msg) => {
                        // Send the message to the broadcast channel
                        if let Err(e) = broadcast().send(TextPayload::ChatMessage(msg)) {
                            error!("Error broadcasting message: {}", e);
                        }
                    }

                    _ => {}
                },
                Err(e) => {
                    error!("Error while parsing text payload: {}", e);
                }
            },

            _ => {}
        }
    }
}

// Other existing functions (parse_command, etc.)
