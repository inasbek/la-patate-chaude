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
    SubscribeResult(SubscribeResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
}

//#[derive(Debug, Serialize, Deserialize)]
// pub struct PublicLeaderBoard {
//     .O: Vec<PublicPlayer>,
// }

