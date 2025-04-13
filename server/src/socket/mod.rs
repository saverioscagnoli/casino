use crate::error::Error;
use fastwebsockets::WebSocketError;
use fastwebsockets::upgrade;
use http_body_util::Empty;
use hyper::Request;
use hyper::Response;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use traccia::{error, info};

mod client;

async fn server_upgrade(
    mut req: Request<Incoming>,
    addr: String,
) -> Result<Response<Empty<Bytes>>, WebSocketError> {
    let (res, upgrade) = upgrade::upgrade(&mut req)?;

    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(client::handler(upgrade, addr)).await {
            error!("Error in websocket connection: {}", e);
        }
    });

    Ok(res)
}

pub async fn task(addr: String, port: u16) -> Result<(), Error> {
    let addr: SocketAddr = format!("{}:{}", addr, port)
        .parse()
        .map_err(|_| Error::AddressParsing(addr))?;

    let listener = TcpListener::bind(addr).await?;

    info!("Server started, listening on {}", addr);

    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client connected from {}", addr);

        tokio::spawn(async move {
            let io = hyper_util::rt::TokioIo::new(stream);
            let conn = http1::Builder::new()
                .serve_connection(io, service_fn(|req| server_upgrade(req, addr.to_string())))
                .with_upgrades();

            if let Err(e) = conn.await {
                error!("Error occured in the socket process: {:?}", e);
            }
        });
    }
}
