use std::str::FromStr;

use console::{
    Command, CommandExecutor, Console, async_trait,
    op::{Clear, ClearKind, PrintLn},
};
use tokio::io::Stdout;
use traccia::{LogLevel, info, log, warn};

struct ClearCommand;

#[async_trait]
impl Command for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }

    fn description(&self) -> &str {
        "Clears the console"
    }

    async fn execute(&mut self, stdout: &mut Stdout, _args: Vec<&str>) -> tokio::io::Result<()> {
        stdout.execute(Clear(ClearKind::All)).await
    }
}

struct LogCommand;

#[async_trait]
impl Command for LogCommand {
    fn name(&self) -> &str {
        "log"
    }

    fn description(&self) -> &str {
        "Logs a message to the console"
    }

    async fn execute(&mut self, _stdout: &mut Stdout, args: Vec<&str>) -> tokio::io::Result<()> {
        if args.is_empty() {
            warn!("Nothing to log.");
            return Ok(());
        }

        let level = LogLevel::from_str(args[0]);

        let (level, message) = match level {
            Ok(level) if args.len() > 1 => (Some(level), args[1..].join(" ")),
            _ => (None, args.join(" ")),
        };

        if message.trim().is_empty() {
            warn!("Nothing to log.");
            return Ok(());
        }

        match level {
            Some(lvl) => log!(lvl, "{}", message),
            None => info!("{}", message),
        }

        Ok(())
    }
}

struct HelpCommand;

impl HelpCommand {
    const COMMANDS: &[&dyn Command] = &[&HelpCommand, &LogCommand, &ClearCommand];
}

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Displays information for all the commands"
    }

    async fn execute(&mut self, stdout: &mut Stdout, _args: Vec<&str>) -> tokio::io::Result<()> {
        let mut text = String::new();

        for command in Self::COMMANDS {
            text.push_str(&format!("{} - {}\n", command.name(), command.description()));
        }

        stdout.execute(PrintLn(text)).await
    }
}

#[tokio::main]
async fn main() {
    traccia::init(LogLevel::Trace);

    _ = Console::new()
        .command(ClearCommand)
        .command(LogCommand)
        .command(HelpCommand)
        .default_callback(|mut stdout, bad| async move {
            stdout
                .execute(PrintLn(format!("Unknown command '{}'", bad)))
                .await?;

            Ok(())
        })
        .prompt("> ")
        .run()
        .await
}
