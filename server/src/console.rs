use std::io::Write;

use crate::{
    json::{self, TextPayload, broadcast},
    ws,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::broadcast,
};
use traccia::{error, info};

enum ConsoleCommand {
    /// Shut the server down
    Exit,
    /// List all commands
    Help,
    /// Displays how many users are connected
    ListUsers,
    /// Clears the console
    Clear,
    /// Sends a chat message to all clients.
    Broadcast(String),
    /// Fallback
    Unknown(String),
}

/// Clears the console
pub fn clear() {
    // Clear screen and move cursor to top-left position
    print!("\x1B[2J\x1B[H");
}

/// Prints the prompt
/// and flushes stdout
pub fn print_prompt() {
    print!("> ");

    if let Err(e) = std::io::stdout().flush() {
        error!("Failed to flush stdout: {}", e);
    }
}

/// Useful for clearing lines before printing logs
pub fn clear_line() {
    print!("\r\x1B[K"); // \r moves cursor to start of line, \x1B[K clears to end of line    print!("\r\x1B[K");

    if let Err(e) = std::io::stdout().flush() {
        error!("Failed to flush stdout: {}", e);
    }
}

fn parse_command(input: &str) -> ConsoleCommand {
    let input = input.trim().to_lowercase();

    match input.as_str() {
        "exit" => ConsoleCommand::Exit,
        "help" => ConsoleCommand::Help,
        "list" => ConsoleCommand::ListUsers,
        "clear" => ConsoleCommand::Clear,
        s if s.starts_with("broadcast ") => {
            let message = s["broadcast ".len()..].trim().to_string();
            if !message.is_empty() {
                ConsoleCommand::Broadcast(message)
            } else {
                ConsoleCommand::Unknown(s.to_string())
            }
        }

        other => ConsoleCommand::Unknown(other.to_string()),
    }
}

// Externalized console input handler with prompt
pub async fn console_task(shutdown_tx: broadcast::Sender<()>) {
    let mut stdout = tokio::io::stdout();
    let mut stdin = BufReader::new(tokio::io::stdin());

    loop {
        if let Err(e) = stdout.flush().await {
            error!("Failed to flush stdout: {}", e);
        }

        // Read input line
        let mut input = String::new();

        match stdin.read_line(&mut input).await {
            Ok(_) => {
                let command = parse_command(&input);

                match command {
                    ConsoleCommand::Exit => {
                        let _ = shutdown_tx.send(());
                        break;
                    }
                    ConsoleCommand::Help => {
                        println!("Available commands:");
                        println!("  help        - Show this help message");
                        println!("  exit        - Shut down the server");
                        println!("  list        - List connected users");
                        println!("  broadcast <message> - Send a message to all connected clients");
                        println!("  clear       - Clear the console");

                        print_prompt();
                    }

                    ConsoleCommand::ListUsers => {
                        let users = ws::client_count();
                        info!("Connected users: {}", users);
                    }

                    ConsoleCommand::Clear => {
                        clear();
                        print_prompt();
                    }

                    ConsoleCommand::Broadcast(message) => {
                        info!("Broadcasting message: {}", message);
                        // Create a system message
                        let payload = TextPayload::ChatMessage(json::ChatMessage {
                            author_id: "system".to_string(),
                            content: message,
                        });

                        if let Err(e) = broadcast().send(payload) {
                            error!("Error broadcasting message: {}", e);
                        }
                    }

                    ConsoleCommand::Unknown(cmd) => {
                        println!(
                            "Unknown command: '{}'. Type 'help' for available commands.",
                            cmd
                        );

                        print_prompt();
                    }
                }
            }
            Err(e) => {
                error!("Failed to read line: {}", e);
                break;
            }
        }
    }
}
