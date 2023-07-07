pub mod game;
pub mod user;

use super::{general::{GameStatus, Country}, events::Visibility};


impl GameStatus {
    pub fn to_string(&self) -> String {
        match self {
            Self::Waiting => "Waiting",
            Self::Started => "Started",
            Self::FirstResigned => "FirstResigned",
            Self::SecondResigned => "SecondResigned",
            Self::FirstWon => "FirstWon",
            Self::SecondWon => "SecondWon",
            Self::FirstDrawOffer => "FirstDrawOffer",
            Self::SecondDrawOffer => "SecondDrawOffer",
            Self::Draw => "Draw",
        }.to_string()
    }

    pub fn from_str(string: &str) -> Self {
        match string {
            "Waiting" => Self::Waiting,
            "Started" => Self::Started,
            "FirstResigned" => Self::FirstResigned,
            "SecondResigned" => Self::SecondResigned,
            "FirstWon" => Self::FirstWon,
            "SecondWon" => Self::SecondWon,
            "Draw" => Self::Draw,
            _ => Self::Waiting,
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
