use async_trait::async_trait;
use console::{AsyncExecute, Clear, PrintLn};
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
