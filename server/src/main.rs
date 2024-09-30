use tokio::{net::TcpListener, sync::Mutex};

use std::sync::Arc;

use crate::{error::Result, handle_connection::handle_connection, mc_server::McServer};

mod error;
mod handle_connection;
mod log;
mod mc_server;
mod process;
mod utils;

const LISTENER_PORT: &str = "0.0.0.0:25560"; // for docker container

#[tokio::main]
async fn main() -> Result<()> {
    let mc_server = Arc::new(Mutex::new(McServer::new()));

    let tcp_listener = TcpListener::bind(LISTENER_PORT).await?;
    loop {
        let (stream, _) = tcp_listener.accept().await.unwrap();
        let mc_server_clone = mc_server.clone();
        tokio::spawn(async move { handle_connection(stream, mc_server_clone).await });
    }
}
