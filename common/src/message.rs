pub use macros::Message;
use serde::{Deserialize, Serialize};

pub trait Message: Serialize + for<'de> Deserialize<'de> {
    fn ser(&self) -> Vec<u8>;
    fn deser(encoded: &Vec<u8>) -> Self
    where
        Self: Sized;
}
