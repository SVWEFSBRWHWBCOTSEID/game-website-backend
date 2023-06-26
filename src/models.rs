use serde::{Deserialize, Serialize};
use uuid::Uuid;


// general game seek struct
#[derive(Deserialize, Serialize)]
pub struct Seek {
    pub rated: bool,
    pub time: u32,
    pub increment: u32,
    pub side: Side,
    pub rating_min: u16,
    pub rating_max: u16,
    pub player: Player,
    pub start_pos: String,
}

// general full game struct
#[derive(Deserialize, Serialize)]
pub struct Game {
    pub id: Uuid,
    pub rated: bool,
    pub game: GameType,
    pub clock: Clock,
    pub created_at: i64,
    pub first: Player,
    pub second: Player,
    pub start_pos: String,
    pub state: GameState,
}

// key and name of game
#[derive(Deserialize, Serialize)]
pub struct GameType {
    pub key: String,
    pub name: String,
}

// time control for game
#[derive(Deserialize, Serialize)]
pub struct Clock {
    pub initial: u32,
    pub increment: u32,
}

// player information for one game
#[derive(Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub provisional: bool,
    pub rating: u32,
}

// general game state for any game
#[derive(Deserialize, Serialize)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub first_time: u32,
    pub second_time: u32,
    pub status: GameStatus,
}

// general move struct
#[derive(Deserialize, Serialize)]
pub struct Move {
    pub user_move: String,
}

// enum for starting side choice
#[derive(Deserialize, Serialize)]
pub enum Side {
    First,
    Second,
    Random,
}

// enum for game status
#[derive(Deserialize, Serialize)]
pub enum GameStatus {
    Started,
}

