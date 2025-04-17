use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

mod request;
mod response;
mod traits;

pub use async_trait::async_trait;
pub use request::Request;
pub use response::Response;
pub use traits::ApiHandler;

pub struct App {
    listener: TcpListener,
}

impl App {
    pub async fn new(addr: SocketAddr) -> tokio::io::Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener })
    }

    pub async fn run<H: ApiHandler>(self, handler: H) -> tokio::io::Result<()> {
        let handler = Arc::new(handler);

        loop {
            let (stream, _) = self.listener.accept().await?;
            let io = TokioIo::new(stream);
            let handler = handler.clone();

            let service = service_fn(move |req| {
                let handler = handler.clone();
                async move {
                    match handler.incoming(req.into()).await {
                        Ok(response) => Ok::<_, hyper::Error>(response.into()),
                        Err(err) => Err(err),
                    }
                }
            });

            tokio::spawn(async move {
                if let Err(e) = http1::Builder::new().serve_connection(io, service).await {
                    eprintln!("Error serving connection: {:?}", e);
                };
            });
        }
    }
}
