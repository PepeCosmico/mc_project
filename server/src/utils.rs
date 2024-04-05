use tokio::net::TcpStream;

use common::{message::send_message, response::Response};

use crate::error::Result;

pub async fn send_response(stream: &mut TcpStream, response: &Response) -> Result<()> {
    send_message(stream, response).await?;
    Ok(())
}
