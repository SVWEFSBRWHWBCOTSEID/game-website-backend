use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use serde::{Deserialize, Serialize};
use crate::models::general::{Country, Preferences};

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
    pub game_key: GameKey,
    pub rated: bool,
    pub time: Option<i32>,
    pub increment: Option<i32>,
    pub side: Side,
    pub start_pos: Option<String>,
}

#[derive(Debug, MultipartForm)]
pub struct ProfileReq {
    pub country: Text<Country>,
    pub location: Text<String>,
    pub bio: Text<String>,

    #[multipart(rename = "firstName")]
    pub first_name: Text<String>,

    #[multipart(rename = "lastName")]
    pub last_name: Text<String>,

    pub pfp: Option<TempFile>,
}
