use serde::{Deserialize, Serialize};

use super::{general::{GameType, TimeControl, Player, Profile, Perfs, Side}, events::GameState};


#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameResponse {
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
pub struct GameResponse {
    pub rated: bool,
    pub game: GameType,
    pub time_control: TimeControl,
    pub created_at: String,
    pub first: Option<Player>,
    pub second: Option<Player>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LobbyResponse {
    pub id: String,
    pub rated: bool,
    pub rating_min: i32,
    pub rating_max: i32,
    pub side: Side,
    pub user: Player,
    pub game: GameType,
    pub time_control: TimeControl,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub username: String,
    pub created_at: String,
    pub guest: bool,
    pub perfs: Perfs,
    pub profile: Profile,
    pub url: String,
    pub playing: Option<String>,
    pub games: Vec<GameResponse>,
}

#[derive(Deserialize, Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OkResponse {
    pub ok: bool,
}

pub static OK_RES: OkResponse = OkResponse {
    ok: true,
};
