use std::net::SocketAddr;

use async_trait::async_trait;
use console::{AsyncExecute, Clear, PrintLn};
use shared::ConcurrentHashMap;
use tokio::io;

/// Help command
/// Displays a list of available commands and their descriptions
///
/// Takes a vector of tuples containing command names and descriptions
/// Easily retrievable with `Command::name()` and `Command::description()`
pub struct HelpCommand(pub Vec<(&'static str, &'static str)>);

#[async_trait]
impl console::Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Displays a list of available commands and their descriptions"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _: &[&str]) -> tokio::io::Result<()> {
        for (name, description) in &self.0 {
            stdout
                .execute(PrintLn(format!("{}: {}", name, description)))
                .await?;
        }

        Ok(())
    }
}

pub struct ClearCommand;

#[async_trait]
impl console::Command for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }

    fn description(&self) -> &str {
        "Clears the console"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _: &[&str]) -> tokio::io::Result<()> {
        stdout.execute(Clear).await?;
        Ok(())
    }
}

pub struct RelayCommand(pub ConcurrentHashMap<SocketAddr, reqwest::Client>);

#[async_trait]
impl console::Command for RelayCommand {
    fn name(&self) -> &str {
        "relay"
    }

    fn description(&self) -> &str {
        "Manages relay connections"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: &[&str]) -> tokio::io::Result<()> {
        match args {
            ["add"] => {
                stdout.execute(PrintLn("Usage: relay add <ipaddr>")).await?;
            }

            ["add", addr, ..] => {
                let client = reqwest::Client::new();
                let addr = match addr.parse::<SocketAddr>() {
                    Ok(addr) => addr,
                    Err(_) => {
                        stdout.execute(PrintLn("Invalid address!")).await?;
                        return Ok(());
                    }
                };

                let relay = format!("http://{}/healthcheck", addr);

                match client.get(&relay).send().await {
                    Ok(_) => {
                        self.0.insert(addr, client);
                        stdout
                            .execute(PrintLn(format!("Successfully added relay: {}", addr)))
                            .await?;
                    }

                    Err(e) => {
                        stdout
                            .execute(PrintLn(format!("Failed to connect to relay: {}", e)))
                            .await?;
                    }
                }
            }

            _ => {
                stdout.execute(PrintLn("Usage: relay <command>")).await?;
            }
        }

        Ok(())
    }
}
