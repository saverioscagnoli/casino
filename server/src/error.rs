use std::fmt::Display;

use tokio_tungstenite::tungstenite;

#[derive(Debug)]
pub enum Error {
    WebSocket(tungstenite::Error),
    Serialization(serde_json::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WebSocket(err) => write!(f, "WebSocket error: {}", err),
            Error::Serialization(err) => write!(f, "Serde error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<tungstenite::Error> for Error {
    fn from(err: tungstenite::Error) -> Self {
        Self::WebSocket(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serialization(err)
    }
}
