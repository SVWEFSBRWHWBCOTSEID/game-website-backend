use serde::{Deserialize, Serialize};
use uuid::Uuid;


// general game seek struct
#[derive(Deserialize, Serialize)]
pub struct Seek {
    pub rated: bool,
    pub time: i32,
    pub increment: i32,
    pub side: crate::prisma::Side,
    pub rating_min: i32,
    pub rating_max: i32,
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
    pub first: Option<Player>,
    pub second: Option<Player>,
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
    pub initial: i32,
    pub increment: i32,
}

// player information for one game
#[derive(Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub provisional: bool,
    pub rating: i32,
}

// general game state for any game
#[derive(Deserialize, Serialize)]
pub struct GameState {
    pub moves: Vec<Move>,
    pub first_time: i32,
    pub second_time: i32,
    pub status: crate::prisma::GameStatus,
}

// general move struct
#[derive(Deserialize, Serialize)]
pub struct Move {
    pub user_move: String,
}
