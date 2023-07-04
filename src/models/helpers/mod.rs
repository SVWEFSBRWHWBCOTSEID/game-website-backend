pub mod game;
pub mod user;

use super::general::{GameStatus, Country};


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
