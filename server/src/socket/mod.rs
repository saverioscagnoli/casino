use crate::error::Error;
use fastwebsockets::WebSocketError;
use fastwebsockets::upgrade;
use futures_util::future;
use futures_util::future::Either;
use http_body_util::Empty;
use hyper::Request;
use hyper::Response;
use hyper::body::Bytes;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use traccia::warn;
use traccia::{error, info};

pub use client::client_count;

mod client;
mod opcode;
mod ws;

async fn server_upgrade(
    mut req: Request<Incoming>,
    addr: String,
    exit_tx: broadcast::Sender<()>,
) -> Result<Response<Empty<Bytes>>, WebSocketError> {
    let (res, upgrade) = upgrade::upgrade(&mut req)?;

    tokio::task::spawn(async move {
        if let Err(e) = tokio::task::unconstrained(client::handler(upgrade, addr, exit_tx)).await {
            error!("Error in websocket connection: {}", e);
        }
    });

    Ok(res)
}

pub async fn task(addr: String, port: u16, exit_tx: broadcast::Sender<()>) -> Result<(), Error> {
    let addr: SocketAddr = format!("{}:{}", addr, port)
        .parse()
        .map_err(|_| Error::AddressParsing(addr))?;

    let listener = TcpListener::bind(addr).await?;

    info!("Server started, listening on {}", addr);

    let mut exit_rx = exit_tx.subscribe();

    loop {
        let accept = listener.accept();
        let exit = exit_rx.recv();

        tokio::pin!(accept);
        tokio::pin!(exit);

        match future::select(accept, exit).await {
            Either::Left((res, _)) => match res {
                Ok((stream, addr)) => {
                    info!("Client connected from {}", addr);

                    let exit_tx = exit_tx.clone();

                    tokio::spawn(async move {
                        let io = hyper_util::rt::TokioIo::new(stream);
                        let conn = http1::Builder::new()
                            .serve_connection(
                                io,
                                service_fn({
                                    move |req| {
                                        server_upgrade(req, addr.to_string(), exit_tx.clone())
                                    }
                                }),
                            )
                            .with_upgrades();

                        if let Err(e) = conn.await {
                            error!("Error occured in the socket process: {:?}", e);
                        }
                    });
                }

                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                }
            },

            _ => {
                warn!("Exit signal was sent. Shutting down...");
                break;
            }
        }
    }

    Ok(())
}
