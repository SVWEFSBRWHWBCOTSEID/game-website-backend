use serde::{Deserialize, Serialize};

use crate::prisma::GameStatus;


// struct for temporary use for player matching
pub struct MatchPlayer {
    pub name: String,
    pub provisional: bool,
    pub rating: i32,
    pub rating_min: i32,
    pub rating_max: i32,
    pub first: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub name: String,
    pub provisional: bool,
    pub rating: i32,
}

#[derive(Deserialize, Serialize)]
pub struct GameType {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Clock {
    pub initial: Option<i32>,
    pub increment: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct GameState {
    pub moves: String,
    pub first_time: Option<i32>,
    pub second_time: Option<i32>,
    pub status: GameStatus,
}
