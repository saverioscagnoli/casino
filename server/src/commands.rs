use crate::{
    console::{self, CommandHandler},
    socket,
};
use async_trait::async_trait;
use std::any::Any;
use strum_macros::EnumIter;
use tokio::{io::AsyncWriteExt, sync::broadcast};
use traccia::{error, info};

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Command {
    Help,
    Exit,
    Count,
    Clear,
}

impl Command {
    fn help(commands: Vec<Command>) {
        for cmd in commands {
            println!("{} - {}", cmd.name(), cmd.desc())
        }
    }

    fn count() {
        let count = socket::client_count();
        info!("Clients connected: {}", count);
    }

    async fn clear(mut stdout: tokio::io::Stdout) {
        if let Err(e) = stdout.write_all(b"\x1b[1;1H\x1b[2J").await {
            error!("Failed to clear the console: {}", e);
        }

        if let Err(e) = stdout.flush().await {
            error!("Failed to flush stdout: {}", e);
        }
    }
}

#[async_trait]
impl console::CommandHandler for Command {
    fn name(&self) -> &str {
        match self {
            Command::Help => "help",
            Command::Exit => "exit",
            Command::Count => "count",
            Command::Clear => "clear",
        }
    }

    fn desc(&self) -> &str {
        match self {
            Command::Help => "Displays information of each command",
            Command::Exit => "Stops the server entirely",
            Command::Count => "Returns the number of clients connected",
            Command::Clear => "Clears the console",
        }
    }

    async fn execute(
        &self,
        _args: Vec<String>,
        stdout: tokio::io::Stdout,
        context: Option<&(dyn Any + Send + Sync)>,
    ) -> Result<(), String> {
        match self {
            Command::Help => {
                if let Some(commands) = context.unwrap().downcast_ref::<Vec<Command>>() {
                    Command::help(commands.clone());
                }
            }
            Command::Count => Command::count(),
            Command::Exit => {
                if let Some(exit_tx) = context.unwrap().downcast_ref::<broadcast::Sender<()>>() {
                    if let Err(e) = exit_tx.send(()) {
                        error!("Failed to send shutdown signal: {}", e);
                    }
                }
            }
            Command::Clear => Command::clear(stdout).await,
        }

        Ok(())
    }
}
