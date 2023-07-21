use crate::common::WebErr;
use crate::models::general::{WinType, DrawOffer, Country, GameKey};
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
            _ => Err(WebErr::NotFound(format!("provided game key does not match an enum variant"))),
        }
    }

    pub fn get_game_name(string: &str) -> Result<String, WebErr> {
        match string {
            "ttt" => Ok("Tic-Tac-Toe".to_string()),
            "uttt" => Ok("ultimate Tic-Tac-Toe".to_string()),
            "c4" => Ok("Connect 4".to_string()),
            "pc" => Ok("Pokémon Chess".to_string()),
            _ => Err(WebErr::NotFound(format!("provided game key does not match an enum variant"))),
        }
    }
}

impl WinType {
    pub fn to_string(&self) -> String {
        match self {
            Self::Normal => "Normal",
            Self::Resign => "Resign",
            Self::Timeout => "Timeout",
            Self::Disconnect => "Disconnect",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Normal" => Self::Normal,
            "Resign" => Self::Resign,
            "Timeout" => Self::Timeout,
            "Disconnect" => Self::Disconnect,
            _ => Self::Normal,
        }
    }
}

impl DrawOffer {
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::None => None,
            Self::First => Some(true),
            Self::Second => Some(false),
        }
    }

    pub fn from_bool(bool: &Option<bool>) -> Self {
        match bool {
            None => Self::None,
            Some(true) => Self::First,
            Some(false) => Self::Second,
        }
    }
}

impl Country {
    pub fn to_string(&self) -> String {
        match self {
            Self::Empty => "Empty",
            Self::Us => "Us",
            Self::Uk => "Uk",
            Self::Mn => "Mn",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Empty" => Self::Empty,
            "Us" => Self::Us,
            "Uk" => Self::Uk,
            "Mn" => Self::Mn,
            _ => Self::Empty,
        }
    }
}

impl Visibility {
    pub fn to_string(&self) -> String {
        match self {
            Self::Player => "Player",
            Self::Spectator => "Spectator",
            Self::Team1 => "Team1",
            Self::Team2 => "Team2",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Player" => Self::Player,
            "Spectator" => Self::Spectator,
            "Team1" => Self::Team1,
            "Team2" => Self::Team2,
            _ => Self::Player,
        }
    }
}
