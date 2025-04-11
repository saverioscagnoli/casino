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
    /// Sends a chat message to all clients.
    Broadcast(String),
    /// Fallback
    Unknown(String),
}

fn parse_command(input: &str) -> ConsoleCommand {
    let input = input.trim();

    match input {
        "exit" => ConsoleCommand::Exit,
        "help" => ConsoleCommand::Help,
        "list" => ConsoleCommand::ListUsers,
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
        // Display the prompt (> )
        if let Err(e) = stdout.write_all(b"> ").await {
            error!("Failed to write prompt: {}", e);
        }

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
                    }

                    ConsoleCommand::ListUsers => {
                        let users = ws::client_count();
                        info!("Connected users: {}", users);
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
