use futures_util::future::{self, Either};
use std::{any::Any, collections::HashMap, io::Write};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::broadcast,
};
use traccia::error;

mod handler;

pub use handler::CommandHandler;

pub struct Console {
    commands: Vec<Box<dyn CommandHandler>>,
    contexts: HashMap<String, Box<dyn Any + Send + Sync>>,
    prompt: Option<String>,
}

impl Console {
    pub fn new() -> Self {
        Console {
            commands: Vec::new(),
            contexts: HashMap::new(),
            prompt: None,
        }
    }

    pub fn prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn register_command<C: CommandHandler + 'static>(mut self, command: C) -> Self {
        self.commands.push(Box::new(command));
        self
    }

    pub fn register_context<C: CommandHandler, A: Any + Send + Sync>(
        mut self,
        command: C,
        context: A,
    ) -> Self {
        self.contexts
            .insert(command.name().to_owned(), Box::new(context));
        self
    }

    fn find_command(&self, name: &str) -> Option<&Box<dyn CommandHandler>> {
        self.commands.iter().find(|cmd| cmd.name() == name)
    }

    pub async fn task(self, exit_tx: broadcast::Sender<()>) {
        let mut exit_rx = exit_tx.subscribe();

        loop {
            let mut stdin = BufReader::new(tokio::io::stdin());
            let mut stdout = tokio::io::stdout();

            if let Some(ref prompt) = self.prompt {
                if let Err(e) = stdout.write_all(prompt.as_bytes()).await {
                    error!("Error while printing the prompt: {}", e)
                }

                if let Err(e) = stdout.flush().await {
                    error!("Error while flushing stdout: {}", e);
                }
            }

            let mut line = String::new();

            let read_line = stdin.read_line(&mut line);
            let exit = exit_rx.recv();

            tokio::pin!(read_line);
            tokio::pin!(exit);

            // Give exit priority, otherwise it will read the line
            // one last time, making it han
            match future::select(exit, read_line).await {
                Either::Right((res, _)) => match res {
                    Ok(_) => {
                        let line = line.trim().to_string();

                        if line.is_empty() {
                            continue;
                        }

                        let parts = line.split_whitespace().collect::<Vec<_>>();
                        let name = parts[0].to_string();

                        if let Some(cmd) = self.find_command(&name) {
                            let args = parts[1..].iter().map(|s| s.to_string()).collect::<Vec<_>>();
                            let context = self.contexts.get(&name);

                            if let Err(e) = cmd.execute(args, stdout, context.map(|v| &**v)).await {
                                println!("{}", e);
                            }
                        } else {
                            println!("Unknown command: '{}'", name);
                        }
                    }

                    Err(e) => {
                        error!("Failed to read line: {}", e);
                        break;
                    }
                },

                _ => {
                    // Shutdown signal received
                    // Flush to ensure the buffer is empty
                    break;
                }
            }
        }
    }
}
