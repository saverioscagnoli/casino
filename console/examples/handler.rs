use async_trait::async_trait;
use console::{AsyncExecute, Clear, Console, Key};
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::io;

static COUNT: AtomicU32 = AtomicU32::new(0);

struct H;

#[async_trait]
impl console::ConsoleHandler for H {
    async fn on_keypress(&mut self, _: &mut io::Stdout, _: Key) -> io::Result<()> {
        COUNT.fetch_add(1, Ordering::SeqCst);

        Ok(())
    }
}

struct ClearCommand;

#[async_trait]
impl console::Command for ClearCommand {
    fn name(&self) -> &str {
        "clear"
    }

    fn description(&self) -> &str {
        "Clears the console"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _args: &[&str]) -> io::Result<()> {
        stdout.execute(Clear).await?;
        Ok(())
    }
}

pub struct CountCommand;

#[async_trait]
impl console::Command for CountCommand {
    fn name(&self) -> &str {
        "count"
    }

    fn description(&self) -> &str {
        "Prints the number of keypresses"
    }

    async fn execute(&mut self, stdout: &mut io::Stdout, _args: &[&str]) -> io::Result<()> {
        let count = COUNT.load(Ordering::SeqCst);
        stdout
            .execute(console::PrintLn(format!("Keypresses: {}", count)))
            .await?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    Console::new()
        .handler(H)
        .command(ClearCommand)
        .command(CountCommand)
        .run()
        .await?;

    Ok(())
}
