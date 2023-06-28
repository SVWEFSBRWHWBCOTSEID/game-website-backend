use serde::{Deserialize, Serialize};

use crate::prisma::Side;
use super::general::Player;


#[derive(Deserialize, Serialize)]
pub struct CreateGameReq {
    pub rated: bool,
    pub time: Option<i32>,
    pub increment: Option<i32>,
    pub side: Side,
    pub rating_min: i32,
    pub rating_max: i32,
    pub player: Player,
    pub start_pos: String,
}
