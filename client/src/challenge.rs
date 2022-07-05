use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    ChallengeName(ChallengeOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeOutput {
    Ok,
    Fail
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    name: ChallengeAnswer,
    next_target: String
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

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ReportedChallengeResult {
//     name: String,
//     value: JobValue
// }
