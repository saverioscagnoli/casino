use clap::Parser;
use commands::Command;
use console::Console;
use strum::IntoEnumIterator;
use tokio::sync::broadcast;
use traccia::LogLevel;

mod commands;
mod console;
mod consts;
mod error;
mod log;
mod socket;

#[derive(Debug, Parser)]
#[command(author, version)]
struct Args {
    /// The address of the tcp listener
    #[arg(short, long, default_value = "127.0.0.1")]
    addr: String,

    /// The port to listen on
    #[arg(short, long)]
    port: u16,

    /// The log level for logging
    #[arg(short, long, default_value_t = log::default_level())]
    level: LogLevel,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = log::config(args.level);

    log::set_hooks();
    traccia::init_with_config(config);

    let (exit_tx, _) = broadcast::channel::<()>(1);

    let console = Console::new()
        .prompt(consts::CONSOLE_PROMPT)
        .register_command(Command::Help)
        .register_context(Command::Help, Command::iter().collect::<Vec<_>>())
        .register_command(Command::Exit)
        .register_context(Command::Exit, exit_tx.clone())
        .register_command(Command::Count)
        .register_command(Command::Clear);

    let s = tokio::spawn(socket::task(args.addr, args.port, exit_tx.clone()));
    let c = tokio::spawn(console.task(exit_tx.clone()));

    _ = s.await;
    _ = c.await;

    Ok(())
}
