use clap::Parser;
use futures_util::{SinkExt, StreamExt, stream::SplitSink};
use mini_moka::sync::Cache;
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use std::{
    net::SocketAddr,
    sync::{Arc, LazyLock},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, broadcast},
};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::Message};
use traccia::{Colorize, LogLevel, Style, error, info, warn};

// Channel for broadcasting messages
static BROADCAST_TX: LazyLock<broadcast::Sender<TextPayload>> = LazyLock::new(|| {
    let (tx, _) = broadcast::channel::<TextPayload>(100);
    return tx;
});

type Tx = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug, Clone)]
struct Ws(Arc<Mutex<Tx>>);

impl Ws {
    async fn send_text<T: Serialize>(&self, v: &T) {
        _ = self
            .0
            .lock()
            .await
            .send(Message::Text(serde_json::to_string(v).unwrap().into()))
            .await;
    }
}

type ClientMap = Cache<String, Ws>;

static CLIENT_MAP: LazyLock<ClientMap> = LazyLock::new(|| Cache::new(100));

#[derive(Deserialize_repr)]
#[repr(u8)]
enum TextOpcode {
    AssignID = 0,
    ChatMessage = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Hello {
    id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    id: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "opcode", content = "data")]
enum TextPayload {
    #[serde(rename = "0")]
    Hello(Hello),
    #[serde(rename = "1")]
    ChatMessage(ChatMessage),
}

struct CustomFormatter;

impl traccia::Formatter for CustomFormatter {
    fn format(&self, record: &traccia::Record) -> String {
        let gray = traccia::Color::RGB(128, 128, 128);

        format!(
            "[{} {} {}] {}",
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M")
                .to_string()
                .color(gray),
            record.level.default_coloring().to_lowercase(),
            record.target.color(gray).italic(),
            record.message
        )
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
    let args = Args::parse();

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

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("Error during WebSocket handshake: {}", e);
            return;
        }
    };

    let (write, mut read) = ws_stream.split();
    let write = Ws(Arc::new(Mutex::new(write)));
    let id = nanoid!();

    CLIENT_MAP.insert(id.clone(), write.clone());

    let hello = Hello { id: id.clone() };
    let payload = TextPayload::Hello(hello);

    // Send the id so that the user can register
    write.send_text(&payload).await;

    info!("Client connected from: {}; assigned id: {}", addr, id);

    // Subscribe to broadcast channel
    let mut broadcast_rx = BROADCAST_TX.subscribe();

    // Spawn a task to handle broadcast messages
    let client_id = id.clone();
    let client_ws = write.clone();
    tokio::spawn(async move {
        while let Ok(msg) = broadcast_rx.recv().await {
            // Only forward messages that aren't from this client
            if let TextPayload::ChatMessage(ref chat_msg) = msg {
                if chat_msg.id != client_id {
                    client_ws.send_text(&msg).await;
                }
            }
        }
    });

    // Handle incoming messages from this client
    while let Some(Ok(message)) = read.next().await {
        match message {
            Message::Binary(bytes) => {
                info!("received: {}", String::from_utf8(bytes.to_vec()).unwrap())
            }

            Message::Text(raw) => match serde_json::from_str::<TextPayload>(&raw) {
                Ok(payload) => match payload {
                    TextPayload::ChatMessage(_) => {
                        // Broadcast the message using the channel
                        if let Err(e) = BROADCAST_TX.send(payload) {
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

    CLIENT_MAP.invalidate(&id);
    warn!("Connection closed: {}", addr);
}
