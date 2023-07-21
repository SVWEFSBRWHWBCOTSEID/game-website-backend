use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;


pub struct MatchPlayer {
    pub username: String,
    pub provisional: bool,
    pub rating: i32,
    pub rating_min: i32,
    pub rating_max: i32,
    pub first: bool,
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

#[derive(Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamePerf {
    pub games: i32,
    pub rating: i32,
    pub rd: f32,
    pub prog: i32,
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FriendRequest {
    Out,
    In,
    Friend,
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

#[derive(Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MoveOutcome {
    None,
    FirstWin,
    SecondWin,
    Draw,
    Stalemate,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    First,
    Second,
    Random,
}
