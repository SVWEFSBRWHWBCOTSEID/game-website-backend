use std::time::SystemTime;

use actix_session::Session;
use actix_web::{ResponseError, HttpResponse, http::{header::ContentType, StatusCode}, web};
use derive_more::{Display, Error};
use phf::phf_map;

use crate::{prisma::{game, PrismaClient, user, message}, models::general::GameStatus};


static KEY_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "ttt" => "Tic-Tac-Toe",
    "uttt" => "Ultimate Tic-Tac-Toe",
    "c4" => "Connect 4",
    "pc" => "Pokemon Chess",
};

pub fn get_key_name(key: &str) -> Option<String> {
    match KEY_NAMES.get(key) {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

pub fn get_username(session: &Session) -> Option<String> {
    match session.get("username") {
        Ok(o) => o,
        Err(_) => None,
    }
}

pub async fn get_game_by_id(client: &web::Data<PrismaClient>, id: &str) -> Option<game::Data> {
    client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .exec()
        .await
        .unwrap()
}

// same as get_game_by_id but checks checks status and username
pub async fn get_game_by_id_validate(client: &web::Data<PrismaClient>, id: &str, username: &str) -> Option<game::Data> {
    match client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => if GameStatus::from_str(&g.status) != GameStatus::Started ||
            g.first_username.clone().unwrap() != username && g.second_username.clone().unwrap() != username {
            None
        } else {
            Some(g)
        }
        None => None,
    }
}

// same as get_game_by_id but fetches user and chat relations
pub async fn get_game_by_id_with_relations(client: &web::Data<PrismaClient>, id: &str) -> Option<game::Data> {
    client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .with(game::first_user::fetch())
        .with(game::second_user::fetch())
        .with(game::chat::fetch(vec![message::game_id::equals(id.to_string())]))
        .exec()
        .await
        .unwrap()
}

pub async fn get_user_by_username(client: &web::Data<PrismaClient>, username: &str) -> Option<user::Data> {
    client
        .user()
        .find_unique(user::username::equals(username.to_string()))
        .exec()
        .await
        .unwrap()
}

pub fn time_millis() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadRequest,

    #[display(fmt = "not authorized for this action")]
    Unauthorized,
    
    #[display(fmt = "timeout")]
    Timeout,
}

impl ResponseError for CustomError {

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            CustomError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            CustomError::BadRequest => StatusCode::BAD_REQUEST,
            CustomError::Unauthorized => StatusCode::BAD_REQUEST,
            CustomError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
