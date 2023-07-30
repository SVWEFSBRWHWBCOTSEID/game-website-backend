use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchPlayer {
    pub username: String,
    pub provisional: bool,
    pub rating: f64,
    pub rating_min: i32,
    pub rating_max: i32,
    pub first: bool,
    pub random: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub username: String,
    pub provisional: bool,
    pub rating: i32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameType {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeControl {
    pub initial: Option<i32>,
    pub increment: Option<i32>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Perfs {
    pub ttt: GamePerf,
    pub uttt: GamePerf,
    pub c4: GamePerf,
    pub pc: GamePerf,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GamePerf {
    pub games: i32,
    pub rating: f64,
    pub rd: f64,
    pub volatility: f64,
    pub tau: f64,
    pub prog: Vec<f64>,
    pub prov: bool,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub country: Country,
    pub location: String,
    pub bio: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Serialize, EnumIter)]
pub enum GameKey {
    TTT,
    UTTT,
    C4,
    PC,
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
pub enum FriendRequest {
    Pending,
    Accepted,
    Removed,
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
pub enum EndType {
    Normal,
    Resign,
    Timeout,
    Disconnect,
    Stalemate,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Offer {
    None,
    First,
    Second,
    Agreed,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoveOutcome {
    None,
    FirstWin,
    SecondWin,
    Draw,
    Stalemate,
}

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    First,
    Second,
    Random,
}
