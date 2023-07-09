pub mod game;
pub mod user;

use super::{general::{GameStatus, Country, WinType, DrawOffer}, events::Visibility};


impl GameStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Waiting => "Waiting",
            Self::Started => "Started",
            Self::FirstWon => "FirstWon",
            Self::SecondWon => "SecondWon",
            Self::Draw => "Draw",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Waiting" => Self::Waiting,
            "Started" => Self::Started,
            "FirstWon" => Self::FirstWon,
            "SecondWon" => Self::SecondWon,
            "Draw" => Self::Draw,
            _ => Self::Waiting,
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
