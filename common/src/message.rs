use serde::{Deserialize, Serialize};

use crate::instructions::Instructions;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub msg: String,
    pub instruc: Instructions,
}
