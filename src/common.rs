use actix_web::{ResponseError, HttpResponse, http::{header::ContentType, StatusCode}};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadRequest,

    #[display(fmt = "not authorized for this action")]
    Unauthorized,

    #[display(fmt = "action forbidden")]
    Forbidden,

    #[display(fmt = "resource not found")]
    NotFound,
    
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
            CustomError::Unauthorized => StatusCode::UNAUTHORIZED,
            CustomError::Forbidden => StatusCode::FORBIDDEN,
            CustomError::NotFound => StatusCode::NOT_FOUND,
            CustomError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}
