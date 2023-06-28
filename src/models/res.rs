use serde::{Deserialize, Serialize};

use super::general::{GameType, Clock, Player, GameState};


#[derive(Deserialize, Serialize)]
pub struct GameResponse {
    pub id: String,
    pub created_at: String,
    pub rated: bool,
    pub game: GameType,
    pub clock: Clock,
    pub first_player: Option<Player>,
    pub second_player: Option<Player>,
    pub start_pos: String,
    pub game_state: GameState,
}
