use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use tokio::sync::broadcast;

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum TextOpcode {
    AssignID = 0,
    ChatMessage = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hello {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    #[serde(rename = "authorID")]
    pub author_id: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "opcode", content = "data")]
pub enum TextPayload {
    #[serde(rename = "0")]
    Hello(Hello),
    #[serde(rename = "1")]
    ChatMessage(ChatMessage),
}

impl TextPayload {
    pub fn hello(id: String) -> TextPayload {
        Self::Hello(Hello { id })
    }
}

// Channel for broadcasting messages
static BROADCAST_TX: LazyLock<broadcast::Sender<TextPayload>> = LazyLock::new(|| {
    let (tx, _) = broadcast::channel::<TextPayload>(100);
    return tx;
});

pub fn broadcast() -> &'static broadcast::Sender<TextPayload> {
    &BROADCAST_TX
}
