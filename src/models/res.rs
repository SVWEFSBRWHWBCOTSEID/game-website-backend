use serde::{Deserialize, Serialize};

use super::general::{GameType, Clock, Player, GameState};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    pub id: String,
    pub created_at: String,
    pub rated: bool,
    pub game: GameType,
    pub clock: Clock,
    pub first_player: Option<Player>,
    pub second_player: Option<Player>,
    pub start_pos: Option<String>,
    pub game_state: GameState,
}
