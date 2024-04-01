pub use macros::Message;
use serde::{Deserialize, Serialize};

pub trait Message: Serialize + for<'de> Deserialize<'de> {
    fn ser(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    fn deser(encoded: Vec<u8>) -> Self {
        bincode::deserialize(&encoded).unwrap()
    }
}
