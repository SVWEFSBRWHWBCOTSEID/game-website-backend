use serde::{Deserialize, Serialize};


pub struct MatchPlayer {
    pub username: String,
    pub provisional: bool,
    pub rating: i32,
    pub rating_min: i32,
    pub rating_max: i32,
    pub first: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Player {
    pub username: String,
    pub provisional: bool,
    pub rating: i32,
}

#[derive(Deserialize, Serialize)]
pub struct GameType {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct TimeControl {
    pub initial: Option<i32>,
    pub increment: Option<i32>,
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

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameStatus {
    Waiting,
    Started,
    FirstWon,
    SecondWon,
    Draw,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WinType {
    Normal,
    Resign,
    Timeout,
    Disconnect,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DrawOffer {
    None,
    First,
    Second,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    First,
    Second,
    Random,
}
