use std::{collections::HashMap, f32::consts::E, net::SocketAddr};

use clap::builder::Str;
use console::{
    Command, CommandExecutor, async_trait,
    op::{Clear, ClearKind, PrintLn},
};
use mini_moka::sync::{Cache, ConcurrentCacheExt};
use tokio::io;

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

pub struct AddRelayComand(pub Cache<String, String>);

#[async_trait]
impl Command for AddRelayComand {
    fn name(&self) -> &str {
        "add-relay"
    }

    fn description(&self) -> &str {
        "Adds a relay server"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: Vec<&str>) -> io::Result<()> {
        if args.is_empty() || args.len() != 1 {
            stdout
                .execute(PrintLn("Usage: add-relay <address>"))
                .await?;

            return Ok(());
        }

        if let Ok(addr) = args[0].parse::<SocketAddr>() {
            self.0.insert(addr.to_string(), "ziope".to_string());
        } else {
            stdout
                .execute(PrintLn(format!("Invalid address: {}", args[0])))
                .await?;
        }

        Ok(())
    }
}

pub struct ListRelaysCommand(pub Cache<String, String>);
#[async_trait]
impl Command for ListRelaysCommand {
    fn name(&self) -> &str {
        "list-relays"
    }

    fn description(&self) -> &str {
        "Displays the current active relays"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _args: Vec<&str>) -> io::Result<()> {
        self.0.sync(); // Ensure the cache is up-to-date

        let mut text = String::new();

        for entry in self.0.iter() {
            text.push_str(&format!("Relay: {} -> {}\n", entry.key(), entry.value()));
        }

        if text.is_empty() {
            text.push_str("No active relays found.\n");
        }

        stdout.execute(PrintLn(text)).await
    }
}
