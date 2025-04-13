use std::fmt::Display;

use fastwebsockets::WebSocketError;

#[derive(Debug)]
pub enum Error {
    AddressParsing(String),
    TokioIo(tokio::io::Error),
    WebSocket(fastwebsockets::WebSocketError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AddressParsing(addr) => write!(f, "Failed to parse socket address: {}", addr),
            Error::TokioIo(err) => write!(f, "Tokio IO error: {}", err),
            Error::WebSocket(err) => write!(f, "Websocket error: {}", err),
        }
    }
}

impl std::error::Error for Error {}

impl From<tokio::io::Error> for Error {
    fn from(err: tokio::io::Error) -> Self {
        Error::TokioIo(err)
    }
}

impl From<WebSocketError> for Error {
    fn from(err: WebSocketError) -> Self {
        Error::WebSocket(err)
    }
}
