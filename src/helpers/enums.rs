use crate::common::WebErr;
use crate::models::general::GameKey;
use crate::models::events::Visibility;


impl GameKey {
    pub fn to_string(&self) -> String {
        match self {
            GameKey::TTT => "ttt",
            GameKey::UTTT => "uttt",
            GameKey::C4 => "c4",
            GameKey::PC => "pc",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Result<Self, WebErr> {
        match string {
            "ttt" => Ok(GameKey::TTT),
            "uttt" => Ok(GameKey::UTTT),
            "c4" => Ok(GameKey::C4),
            "pc" => Ok(GameKey::PC),
            _ => Err(WebErr::NotFound(format!("provided game key string does not match an enum variant"))),
        }
    }

    pub fn get_game_name(string: &str) -> Result<String, WebErr> {
        match string {
            "ttt" => Ok("Tic-Tac-Toe".to_string()),
            "uttt" => Ok("Ultimate Tic-Tac-Toe".to_string()),
            "c4" => Ok("Connect 4".to_string()),
            "pc" => Ok("Pokémon Chess".to_string()),
            _ => Err(WebErr::NotFound(format!("provided game key string does not match an enum variant"))),
        }
    }
}

impl Visibility {
    pub fn caps_to_pascal(string: &String) -> Result<String, WebErr> {
        match string.as_str() {
            "PLAYER" => Ok("Player".to_string()),
            "SPECTATOR" => Ok("Spectator".to_string()),
            "TEAM1" => Ok("Team1".to_string()),
            "TEAM2" => Ok("Team2".to_string()),
            _ => Err(WebErr::NotFound(format!("provided visibility string {} does not match an all caps enum variant", string))),
        }
    }
}
