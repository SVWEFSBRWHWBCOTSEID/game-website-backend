use serde::{Deserialize, Serialize};
use crate::models::general::Preferences;

use super::general::{Side, GameKey};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameReq {
    pub rated: bool,
    pub time: Option<i32>,
    pub increment: Option<i32>,
    pub side: Side,
    pub rating_min: i32,
    pub rating_max: i32,
    pub start_pos: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
    pub preferences: Option<Preferences>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginReq {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageReq {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMessageReq {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChallengeReq {
    pub opponent: String,
    pub game_key: GameKey,
    pub rated: bool,
    pub time: Option<i32>,
    pub increment: Option<i32>,
    pub side: Side,
    pub start_pos: Option<String>,
}
