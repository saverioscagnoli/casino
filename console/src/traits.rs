use async_trait::async_trait;
use tokio::io::{self, AsyncWriteExt};

use crate::input::Key;

pub trait Op {
    fn ansi(&self) -> String;
}

#[async_trait]
pub trait AsyncExecute {
    async fn execute<O: Op + Send + Sync>(&mut self, op: O) -> io::Result<()>;
}

#[async_trait]
impl<W: AsyncWriteExt + Unpin + Send> AsyncExecute for W {
    async fn execute<O: Op + Send + Sync>(&mut self, op: O) -> io::Result<()> {
        let ansi = op.ansi();

        self.write_all(ansi.as_bytes()).await?;
        self.flush().await?;

        Ok(())
    }
}

#[async_trait]
pub trait Command {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&mut self, stdout: &mut io::Stdout, args: &[&str]) -> io::Result<()>;
}

#[async_trait]
pub trait ConsoleHandler {
    async fn on_keypress(&mut self, stdout: &mut io::Stdout, key: Key) -> io::Result<()>;
}
