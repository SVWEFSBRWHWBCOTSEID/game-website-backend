use std::time::SystemTime;
use actix_session::Session;
use actix_web::web;

use crate::common::CustomError;
use crate::models::general::GameStatus;
use crate::prisma::{user, PrismaClient, message, game};


pub fn get_username(session: &Session) -> Result<String, CustomError> {
    match session.get("username") {
        Ok(u) => Ok(u.unwrap()),
        Err(_) => Err(CustomError::Unauthorized),
    }
}

pub async fn get_game_by_id(client: &web::Data<PrismaClient>, id: &str) -> Result<game::Data, CustomError> {
    match client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => Ok(g),
        None => Err(CustomError::NotFound),
    }
}

// same as get_game_by_id but checks checks status and username
pub async fn get_game_validate(client: &web::Data<PrismaClient>, id: &str, username: &str) -> Result<game::Data, CustomError> {
    match client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => if GameStatus::from_str(&g.status) != GameStatus::Started ||
            g.first_username.clone().unwrap() != username && g.second_username.clone().unwrap() != username {
                Err(CustomError::NotFound)
        } else {
            Ok(g)
        }
        None => Err(CustomError::NotFound),
    }
}

// same as get_game_by_id but fetches user and chat relations
pub async fn get_game_with_relations(client: &web::Data<PrismaClient>, id: &str) -> Result<game::Data, CustomError> {
    match client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .with(game::first_user::fetch())
        .with(game::second_user::fetch())
        .with(game::chat::fetch(vec![message::game_id::equals(id.to_string())]))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => Ok(g),
        None => Err(CustomError::NotFound),
    }
}

pub async fn get_user_with_relations(client: &web::Data<PrismaClient>, username: &str) -> Result<user::Data, CustomError> {
    match client
        .user()
        .find_unique(user::username::equals(username.to_string()))
        .with(user::perfs::fetch(vec![]))
        .exec()
        .await
        .unwrap()
    {
        Some(u) => Ok(u),
        None => Err(CustomError::NotFound),
    }
}

pub async fn set_user_playing(client: &web::Data<PrismaClient>, username: &str, playing: Option<String>) -> Result<(), CustomError> {
    client
        .user()
        .update(
            user::username::equals(username.to_string()),
            vec![
                user::playing::set(playing),
            ],
        )
        .exec()
        .await
        .or(Err(CustomError::InternalError))?;
    Ok(())
}

pub fn time_millis() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64
}

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
