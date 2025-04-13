use async_trait::async_trait;
use std::any::Any;

#[async_trait]
pub trait CommandHandler: Send {
    fn name(&self) -> &str;
    fn desc(&self) -> &str;
    async fn execute(
        &self,
        args: Vec<String>,
        stdout: tokio::io::Stdout,
        context: Option<&(dyn Any + Send + Sync)>,
    ) -> Result<(), String>;
}
