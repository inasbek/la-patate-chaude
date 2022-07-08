use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    version: u8,
}

// {welcome{welcome: {version: 1}}}
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChallengeResult {
    pub(crate) answer: ChallengeAnswer,
    pub(crate) next_target: String
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeErr)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeErr {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicPlayer {
    pub name: String,
    pub stream_id: String,
    pub score: i32,
    pub steps: u32,
    pub is_active: bool,
    pub total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard (pub(crate) Vec<PublicPlayer>);

#[derive(Deserialize, Serialize, Debug)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult {
        used_time: f64,
        next_target: String
    },
    Ok {
        used_time: f64,
        next_target: String
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EndOfGame {
    pub leader_board: PublicLeaderBoard
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MD5HashCashInput {
pub(crate) complexity: u32,
pub(crate) message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MD5HashCashOutput {
    pub(crate) seed: u64,
    pub(crate) hashcode: String,
}
