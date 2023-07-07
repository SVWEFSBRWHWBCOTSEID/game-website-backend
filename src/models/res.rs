use serde::{Deserialize, Serialize};

use super::general::{GameType, TimeControl, Player, GameState, Profile, Perfs};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    pub id: String,
    pub created_at: String,
    pub rated: bool,
    pub game: GameType,
    pub time_control: TimeControl,
    pub first_player: Option<Player>,
    pub second_player: Option<Player>,
    pub start_pos: Option<String>,
    pub game_state: GameState,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub username: String,
    pub created_at: String,
    pub perfs: Perfs,
    pub profile: Profile,
    pub url: String,
    pub playing: Option<String>,
}
