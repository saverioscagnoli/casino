use crate::{request::Request, response::Response};
use async_trait::async_trait;

#[async_trait]
pub trait ApiHandler: Send + Sync + 'static {
    async fn incoming(&self, req: Request) -> Result<Response, hyper::Error>;
}
