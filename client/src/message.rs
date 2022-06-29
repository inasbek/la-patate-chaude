use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    version: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
}