use fastwebsockets::{OpCode, upgrade};
use traccia::warn;

use crate::error::Error;

pub async fn handler(upgrade: upgrade::UpgradeFut, addr: String) -> Result<(), Error> {
    let mut ws = fastwebsockets::FragmentCollector::new(upgrade.await?);

    loop {
        let frame = ws.read_frame().await?;

        match frame.opcode {
            OpCode::Close => break,
            OpCode::Text | OpCode::Binary => {
                ws.write_frame(frame).await?;
            }
            _ => {}
        }
    }

    warn!("Client {} disconnected", addr);
    Ok(())
}
