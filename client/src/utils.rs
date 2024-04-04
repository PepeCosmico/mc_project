use tokio::net::TcpStream;

use crate::error::{Error, Result};

const HELLO_MSG: &str = "---[ Welcome to the CumServer ]---";
const LISTENER_PORT: &str = "127.0.0.1:25560";

pub fn print_welcome_msg() {
    println!("{HELLO_MSG}");
}

pub async fn connect_to_server() -> Result<TcpStream> {
    TcpStream::connect(LISTENER_PORT)
        .await
        .map_err(|e| Error::IOError(e))
}
