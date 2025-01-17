use serde::{Deserialize, Serialize};

use crate::message::Message;

#[derive(Serialize, Deserialize, Message, Debug)]
pub struct Response {
    status: bool,
    msg: Option<String>,
}

impl Response {
    pub fn new(status: bool, msg: Option<String>) -> Self {
        Response { status, msg }
    }
    pub fn is_ok(&self) -> bool {
        self.status
    }
    pub fn get_msg(&self) -> Option<String> {
        self.msg.clone()
    }
}
