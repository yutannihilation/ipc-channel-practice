use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Connect(String),
    Ready,
    Message(String),
}
