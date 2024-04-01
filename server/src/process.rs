use std::{io::Write, process::ChildStdin};

use common::instructions::Instruction;

pub fn process_instructions(msg: &Instruction, child_stdin: &mut Option<ChildStdin>) {
    let bytes = msg.as_command();
    child_stdin.as_mut().unwrap().write_all(&bytes).unwrap();
}
