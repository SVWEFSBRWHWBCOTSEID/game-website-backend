pub mod app_config;
pub mod models;
pub mod handlers;
pub mod prisma;

use actix_web::{ResponseError, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};
use phf::phf_map;


static KEY_NAMES: phf::Map<&'static str, &'static str> = phf_map! {
    "ttt" => "Tic-Tac-Toe",
    "uttt" => "Ultimate Tic-Tac-Toe",
    "pc" => "Pokemon Chess",
};

pub fn get_key_name(key: &str) -> String {
    KEY_NAMES.get(key).unwrap().to_string()
}

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadRequest,
    
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
            CustomError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
