use crate::instructions::Instruction;

pub trait Message {
    fn ser(&self) -> Vec<u8>;
    fn deser(val: &Vec<u8>) -> Self
    where
        Self: Sized;
    fn get_type(&self) -> MessageType;
    fn get_instruction(&self) -> &Instruction;
}

pub enum MessageType {
    ServerCommand,
    MinecraftCommand,
}
