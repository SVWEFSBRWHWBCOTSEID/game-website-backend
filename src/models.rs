use serde::{Deserialize, Serialize};
use uuid::Uuid;


// general move struct
#[derive(Deserialize, Serialize)]
pub struct Move {
    user_move: String,
}

// general full game struct
#[derive(Deserialize, Serialize)]
pub struct Game {
    id: Uuid,
    rated: bool,
    game: GameType,
    clock: Clock,
    created_at: u64,
    white: Player,
    black: Player,
    start_pos: String,
    state: GameState,
}

// key and name of game
#[derive(Deserialize, Serialize)]
pub struct GameType {
    key: String,
    name: String,
}

// time control for game
#[derive(Deserialize, Serialize)]
pub struct Clock {
    initial: u32,
    increment: u32,
}

// player information for one game
#[derive(Deserialize, Serialize)]
pub struct Player {
    id: Uuid,
    name: String,
    provisional: bool,
    rating: u32,
}

// general game state for any game
#[derive(Deserialize, Serialize)]
pub struct GameState {
    moves: Vec<Move>,
    white_time: u32,
    black_time: u32,
    status: String,
}

