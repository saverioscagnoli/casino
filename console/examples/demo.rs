use async_trait::async_trait;
use console::{AsyncExecute, Console, PrintLn};
use tokio::io;

struct HelloCommand;

#[async_trait]
impl console::Command for HelloCommand {
    fn name(&self) -> &str {
        "hello"
    }

    fn description(&self) -> &str {
        "Prints 'Hello, World!'"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _args: &[&str]) -> io::Result<()> {
        stdout.execute(PrintLn("Hello, World!")).await?;
        Ok(())
    }
}

struct GreetCommand;

#[async_trait]
impl console::Command for GreetCommand {
    fn name(&self) -> &str {
        "greet"
    }

    fn description(&self) -> &str {
        "Greets the user"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: &[&str]) -> io::Result<()> {
        match &args[..] {
            [] => {
                stdout.execute(PrintLn("Hello!")).await?;
            }

            [name] => {
                stdout.execute(PrintLn(format!("Hello, {}!", name))).await?;
            }

            _ => {
                stdout.execute(PrintLn("Usage: greet [name]")).await?;
            }
        }

        Ok(())
    }
}

struct DefaultCommand;

#[async_trait]
impl console::Command for DefaultCommand {
    fn name(&self) -> &str {
        "default"
    }

    fn description(&self) -> &str {
        "Default command"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, args: &[&str]) -> io::Result<()> {
        if let Some(name) = args.get(0) {
            stdout
                .execute(PrintLn(format!("Unknown command: '{}'", name)))
                .await?;
        } else {
            stdout.execute(PrintLn("Unknown command.")).await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let mut console = Console::new()
        .command(HelloCommand)
        .command(GreetCommand)
        .default_command(DefaultCommand)
        .prompt("-> ");

    _ = console.run().await;
}
