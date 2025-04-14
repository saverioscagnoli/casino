use super::ws::Ws;
use crate::{error::Error, socket::opcode::Opcode};
use fastwebsockets::FragmentCollectorRead;
use fastwebsockets::{OpCode as Message, upgrade};
use futures_util::future::{self, Either};
use mini_moka::sync::{Cache, ConcurrentCacheExt};
use nanoid::nanoid;
use std::sync::{Arc, LazyLock};
use tokio::sync::{Mutex, broadcast};
use traccia::{error, warn};

static CLIENTS: LazyLock<Cache<String, Ws>> = LazyLock::new(|| Cache::new(100));

pub fn client_count() -> u64 {
    CLIENTS.sync();
    CLIENTS.entry_count()
}

pub async fn handler(
    upgrade: upgrade::UpgradeFut,
    addr: String,
    exit_tx: broadcast::Sender<()>,
) -> Result<(), Error> {
    let ws = upgrade.await?;
    let (rx, tx) = ws.split(tokio::io::split);
    let mut rx = FragmentCollectorRead::new(rx);
    let mut exit_rx = exit_tx.subscribe();

    let id = Arc::new(nanoid!());
    let ws = Ws(Arc::new(Mutex::new(tx)));

    CLIENTS.insert(id.to_string(), ws.clone());

    loop {
        let mut send_frame_closure = |frame| {
            let ws = ws.clone();
            async move { ws.send_frame(frame).await }
        };

        let frame = rx.read_frame(&mut send_frame_closure);
        let exit = exit_rx.recv();

        tokio::pin!(frame);
        tokio::pin!(exit);

        match future::select(frame, exit).await {
            Either::Left((res, _)) => match res {
                Ok(frame) => {
                    match frame.opcode {
                        Message::Close => break,
                        Message::Binary => {
                            let opcode = match frame.payload.get(0) {
                                Some(opcode) => *opcode,
                                None => {
                                    warn!("Recevied message with no opcode. Skipping.");
                                    continue;
                                }
                            };

                            match Opcode::try_from(opcode) {
                                Ok(Opcode::Hello) => {
                                    let username = frame.payload.get(1..);

                                    if let Some(username) = username {
                                        let _username =
                                            String::from_utf8(username.to_vec()).unwrap();

                                        let mut payload = vec![Opcode::Hello.into()];
                                        payload.extend_from_slice(id.as_bytes()); // Append the UTF-8 bytes of the id

                                        // Send the binary message
                                        if let Err(e) = ws.send_bytes(payload).await {
                                            error!("Failed to send hello payload back: {}", e);
                                        }
                                    }
                                }

                                Err(_) => {
                                    warn!("Invalid opcode received: {}", opcode);
                                    continue;
                                }
                            }
                        }

                        _ => {}
                    }
                }
                Err(e) => {
                    error!("Error reading frame: {}", e);
                    continue;
                }
            },

            _ => {
                // Shutdown signal was sent
                return Ok(());
            }
        }
    }

    warn!("Client {} disconnected", addr);
    CLIENTS.invalidate(&id);
    Ok(())
}
