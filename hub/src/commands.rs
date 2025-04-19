use std::net::SocketAddr;

use console::{
    Command, CommandExecutor, async_trait,
    op::{Clear, ClearKind, PrintLn},
};
use mini_moka::sync::ConcurrentCacheExt;
use tokio::io;

use crate::ClientMap;

pub struct ClearCommand;

#[async_trait]
impl Command for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }

    fn description(&self) -> &str {
        "Clears the console"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _: Vec<&str>) -> io::Result<()> {
        stdout.execute(Clear(ClearKind::All)).await?;
        Ok(())
    }
}

pub struct RelayCommand(pub ClientMap);

#[async_trait]
impl Command for RelayCommand {
    fn name(&self) -> &str {
        "relay"
    }

    fn description(&self) -> &str {
        "Add / remove relay servers"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: Vec<&str>) -> io::Result<()> {
        match &args[..] {
            ["add"] => stdout.execute(PrintLn("Usage: relay add <ipaddr>")).await?,
            ["add", ip] => match ip.parse() {
                Ok::<SocketAddr, _>(addr) => {
                    let client = reqwest::Client::new();
                    self.0.insert(addr.to_string(), client);

                    stdout.execute(PrintLn("Relay added successfully.")).await?;
                }
                Err(_) => {
                    stdout
                        .execute(PrintLn(format!("Failed to parse address: {}", ip)))
                        .await?
                }
            },

            ["list"] => {
                self.0.sync();

                let mut n = 0;

                for (i, entry) in self.0.iter().enumerate() {
                    stdout
                        .execute(PrintLn(format!("{}) {}", i + 1, entry.key())))
                        .await?;

                    n += 1;
                }

                if n == 0 {
                    stdout
                        .execute(PrintLn("There are no relays connected."))
                        .await?;
                }
            }

            _ => {
                stdout.execute(PrintLn("Usage: relay <op> ...args")).await?;
            }
        }

        Ok(())
    }
}
