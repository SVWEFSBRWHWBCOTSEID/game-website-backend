use serde::{Deserialize, Serialize};


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

#[derive(Deserialize, Serialize)]
pub struct Perfs {
    pub ttt: GamePerf,
    pub uttt: GamePerf,
    pub c4: GamePerf,
    pub pc: GamePerf,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
pub struct GamePerf {
    pub games: i32,
    pub rating: i32,
    pub rd: f32,
    pub prog: i32,
    pub prov: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub country: Country,
    pub location: String,
    pub bio: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Country {
    Empty,
    Us,
    Uk,
    Mn,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStatus {
    Waiting,
    Started,
    FirstResigned,
    SecondResigned,
    FirstWon,
    SecondWon,
    FirstDrawOffer,
    SecondDrawOffer,
    Draw,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    First,
    Second,
    Random,
}
