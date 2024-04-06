use tokio::net::TcpListener;

use std::sync::{Arc, Mutex};

use crate::{error::Result, handle_connection::handle_connection, mc_server::McServer};

mod error;
mod handle_connection;
mod mc_server;
mod process;
mod utils;

const LISTENER_PORT: &str = "127.0.0.1:25560";

#[tokio::main]
async fn main() -> Result<()> {
    // Create server process
    /*
    let mut child = Command::new("java")
        .args(ARGS)
        .stdin(Stdio::piped())
        .current_dir("../serverdata/")
        .spawn()?;
    */
    let mc_server = Arc::new(Mutex::new(McServer::new()));

    let tcp_listener = TcpListener::bind(LISTENER_PORT).await?;
    loop {
        let (stream, _) = tcp_listener.accept().await.unwrap();
        let mc_server_clone = mc_server.clone();
        tokio::spawn(async move { handle_connection(stream, mc_server_clone).await });
    }
}
