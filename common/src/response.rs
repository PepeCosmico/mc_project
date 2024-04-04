use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Message, Debug)]
pub struct Response {
    status: bool,
}

impl Response {
    pub fn new(status: bool) -> Self {
        Response { status }
    }
}
