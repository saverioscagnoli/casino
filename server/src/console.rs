use console::{
    Command, CommandExecutor, async_trait,
    op::{Clear, ClearKind, PrintLn},
};
use mini_moka::sync::ConcurrentCacheExt;
use std::{net::SocketAddr, str::FromStr};
use tokio::io;

use crate::RELAYS;

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

pub struct RelayComand;

#[async_trait]
impl Command for RelayComand {
    fn name(&self) -> &str {
        "relay"
    }

    fn description(&self) -> &str {
        "Edit relay list for this server"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: Vec<&str>) -> io::Result<()> {
        match args[..] {
            ["add", ip] => {
                if let Ok(ip) = SocketAddr::from_str(ip) {
                    RELAYS.insert(ip, "dfd".to_string());
                    stdout
                        .execute(PrintLn(format!(
                            "relay with ip {} was added successfully",
                            ip
                        )))
                        .await?;
                } else {
                    stdout
                        .execute(PrintLn(format!("address {} is not valid.", ip)))
                        .await?;
                }
            }

            ["list"] => {
                RELAYS.sync();

                let mut n = 0;

                for entry in RELAYS.iter() {
                    stdout
                        .execute(PrintLn(format!("{} -> {}", entry.key(), entry.value())))
                        .await?;
                    n += 1;
                }

                if n == 0 {
                    stdout.execute(PrintLn("No relays active.")).await?;
                }
            }

            _ => stdout.execute(PrintLn("Usage: relay <op> ...args")).await?,
        }

        Ok(())
    }
}
