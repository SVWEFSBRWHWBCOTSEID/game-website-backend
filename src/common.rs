use actix_web::{ResponseError, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};
use phf::phf_map;


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
