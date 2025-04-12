use std::sync::LazyLock;

use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;
use tokio::sync::broadcast;

use crate::database::User;

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum TextOpcode {
    AssignID = 0,
    ChatMessage = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hello {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub author: User,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "opcode", content = "data")]
pub enum TextPayload {
    #[serde(rename = "0")]
    Hello(User),
    #[serde(rename = "1")]
    ChatMessage(ChatMessage),
}

// Channel for broadcasting messages
static BROADCAST_TX: LazyLock<broadcast::Sender<TextPayload>> = LazyLock::new(|| {
    let (tx, _) = broadcast::channel::<TextPayload>(100);
    return tx;
});

pub fn broadcast() -> &'static broadcast::Sender<TextPayload> {
    &BROADCAST_TX
}
