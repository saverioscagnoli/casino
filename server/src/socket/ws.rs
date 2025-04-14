use fastwebsockets::{Frame, Payload, WebSocketWrite};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use std::{ops::Deref, sync::Arc};
use tokio::{io::WriteHalf, sync::Mutex};

use crate::error::Error;

pub type Tx = WebSocketWrite<WriteHalf<TokioIo<Upgraded>>>;

#[derive(Clone)]
pub struct Ws(pub Arc<Mutex<Tx>>);

impl Deref for Ws {
    type Target = Arc<Mutex<Tx>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Ws {
    pub async fn send_frame<'a>(&self, frame: Frame<'a>) -> Result<(), Error> {
        let mut lock = self.lock().await;

        lock.write_frame(frame).await?;

        Ok(())
    }

    pub async fn send_bytes(&self, bytes: Vec<u8>) -> Result<(), Error> {
        let mut lock = self.lock().await;
        let payload = Payload::Owned(bytes);
        let frame = Frame::binary(payload);

        lock.write_frame(frame).await?;

        Ok(())
    }
}
