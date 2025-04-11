use crate::{error::Error, json::TextPayload};
use futures_util::{SinkExt, stream::SplitSink};
use mini_moka::sync::{Cache, ConcurrentCacheExt};
use serde::Serialize;
use std::{
    ops::Deref,
    sync::{Arc, LazyLock},
};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub type Tx = SplitSink<WebSocketStream<TcpStream>, Message>;

#[derive(Debug, Clone)]
pub struct Ws(pub Arc<Mutex<Tx>>);

impl Deref for Ws {
    type Target = Arc<Mutex<Tx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ws {
    pub async fn send_hello(&self, id: String) -> Result<(), Error> {
        let payload = TextPayload::hello(id);

        self.send_json(&payload).await?;

        Ok(())
    }

    pub async fn send_json<T: Serialize>(&self, v: &T) -> Result<(), Error> {
        let payload = serde_json::to_string(v)?;
        let message = Message::Text(payload.into());

        let mut lock = self.lock().await;

        lock.send(message).await?;

        Ok(())
    }
}

type ClientMap = Cache<String, Ws>;

static CLIENT_MAP: LazyLock<ClientMap> = LazyLock::new(|| Cache::new(100));

pub fn client_count() -> u64 {
    CLIENT_MAP.sync();
    CLIENT_MAP.entry_count()
}

pub fn insert_client(id: String, ws: Ws) {
    CLIENT_MAP.insert(id, ws);
}

pub fn remove_client(id: &String) {
    CLIENT_MAP.invalidate(id);
}
