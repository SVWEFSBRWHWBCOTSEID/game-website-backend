use crate::common::WebErr;
use crate::models::general::{WinType, DrawOffer, Country, GameKey, FriendRequest};
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
            "uttt" => Ok("ultimate Tic-Tac-Toe".to_string()),
            "c4" => Ok("Connect 4".to_string()),
            "pc" => Ok("Pokémon Chess".to_string()),
            _ => Err(WebErr::NotFound(format!("provided game key string does not match an enum variant"))),
        }
    }
}

impl FriendRequest {
    pub fn to_string(&self) -> String {
        match self {
            FriendRequest::Pending => "Pending",
            FriendRequest::Accepted => "Accepted",
            FriendRequest::Removed => "Remove",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Result<Self, WebErr> {
        match string {
            "Pending" => Ok(FriendRequest::Pending),
            "Accepted" => Ok(FriendRequest::Accepted),
            "Remove" => Ok(FriendRequest::Removed),
            _ => Err(WebErr::NotFound(format!("provided friend request string does not match an enum variant"))),
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

    pub fn from_str(string: &str) -> Result<Self, WebErr> {
        match string {
            "Normal" => Ok(Self::Normal),
            "Resign" => Ok(Self::Resign),
            "Timeout" => Ok(Self::Timeout),
            "Disconnect" => Ok(Self::Disconnect),
            _ => Err(WebErr::NotFound(format!("provided wintype string does not match an enum variant"))),
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

    pub fn from_str(string: &str) -> Result<Self, WebErr> {
        match string {
            "Empty" => Ok(Self::Empty),
            "Us" => Ok(Self::Us),
            "Uk" => Ok(Self::Uk),
            "Mn" => Ok(Self::Mn),
            _ => Err(WebErr::NotFound(format!("provided country string does not match an enum variant"))),
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

    pub fn from_str(string: &str) -> Result<Self, WebErr> {
        match string {
            "Player" => Ok(Self::Player),
            "Spectator" => Ok(Self::Spectator),
            "Team1" => Ok(Self::Team1),
            "Team2" => Ok(Self::Team2),
            _ => Err(WebErr::NotFound(format!("provided visibility string does not match an enum variant"))),
        }
    }
}
