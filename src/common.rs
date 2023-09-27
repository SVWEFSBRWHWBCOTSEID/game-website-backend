use std::error::Error;
use std::fmt::{self, Display};
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::{header::ContentType, StatusCode};
use strum::ParseError;


#[derive(Debug)]
pub enum WebErr {
    Internal(String),
    BadReq(String),
    Unauth(String),
    Forbidden(String),
    NotFound(String),
    Timeout(String),
}

impl Display for WebErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            WebErr::Internal(x) => x,
            WebErr::BadReq(x) => x,
            WebErr::Unauth(x) => x,
            WebErr::Forbidden(x) => x,
            WebErr::NotFound(x) => x,
            WebErr::Timeout(x) => x,
        })
    }
}

impl Error for WebErr {}

impl ResponseError for WebErr {
    fn status_code(&self) -> StatusCode {
        match *self {
            WebErr::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            WebErr::BadReq(_) => StatusCode::BAD_REQUEST,
            WebErr::Unauth(_) => StatusCode::UNAUTHORIZED,
            WebErr::Forbidden(_) => StatusCode::FORBIDDEN,
            WebErr::NotFound(_) => StatusCode::NOT_FOUND,
            WebErr::Timeout(_) => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
}

impl From<ParseError> for WebErr {
    fn from(_: ParseError) -> Self {
        WebErr::NotFound(format!("provided string does not match an enum variant"))
    }
}
