use tokio::net::TcpListener;

use std::{
    process::{Command, Stdio},
    sync::{Arc, Mutex},
};

use error::Result;
use handle_connection::handle_connection;

mod command;
mod error;
mod handle_connection;
mod process;

const ARGS: [&str; 4] = ["-Xmx6G", "-jar", "fabric-server-1.20.4.jar", "nogui"];
const LISTENER_PORT: &str = "127.0.0.1:25560";

#[tokio::main]
async fn main() -> Result<()> {
    // Create server process
    let mut child = Command::new("java")
        .args(ARGS)
        .stdin(Stdio::piped())
        .current_dir("../serverdata/")
        .spawn()?;
    let child_stdin = Arc::new(Mutex::new(child.stdin.take().unwrap()));

    let tcp_listener = TcpListener::bind(LISTENER_PORT).await?;
    loop {
        let (stream, _) = tcp_listener.accept().await.unwrap();
        let child_stdin_cloned = child_stdin.clone();
        tokio::spawn(async move { handle_connection(stream, child_stdin_cloned).await });
    }
}
