use serde::{Deserialize, Serialize};

use crate::prisma::Side;


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameReq {
    pub rated: bool,
    pub time: Option<i32>,
    pub increment: Option<i32>,
    pub side: Side,
    pub rating_min: i32,
    pub rating_max: i32,
    pub username: String,
    pub start_pos: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserReq {
    pub name: String,
}
