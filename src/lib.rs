use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Connect(String),
    Message(String),
}
