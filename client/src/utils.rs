use std::time::Duration;

use common::{
    message::{read_message, send_message, Message},
    response::Response,
};
use tokio::{net::TcpStream, time};

use crate::error::{Error, Result};

const HELLO_MSG: &str = "---[ Welcome to the McServer ]---";
const LISTENER_PORT: &str = "127.0.0.1:25560";

pub fn print_welcome_msg() {
    println!("{HELLO_MSG}");
}

pub async fn connect_to_server() -> Result<TcpStream> {
    TcpStream::connect(LISTENER_PORT)
        .await
        .map_err(|e| Error::IOError(e))
}

pub async fn send_msg_wait_response(
    stream: &mut TcpStream,
    msg: &impl Message,
) -> Result<Response> {
    send_message(stream, msg).await?;
    Ok(
        match time::timeout(Duration::from_secs(5), read_message::<Response>(stream)).await {
            Ok(Ok(response)) => response,
            Ok(Err(e)) => return Err(Error::CommonError(e)),
            Err(_) => return Err(Error::ReadResponseTimeoutError),
        },
    )
}
