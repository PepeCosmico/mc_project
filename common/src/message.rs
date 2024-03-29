use serde::{Deserialize, Serialize};

use crate::instructions::Instruction;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub instruc: Instruction,
}
