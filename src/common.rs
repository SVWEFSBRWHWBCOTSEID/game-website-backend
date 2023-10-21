use std::error::Error;
use std::fmt::{self, Display};
use actix_web::{ResponseError, HttpResponse};
use actix_web::http::{header::ContentType, StatusCode};
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::put_object::PutObjectError;
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

impl From<SdkError<PutObjectError>> for WebErr {
    fn from(e: SdkError<PutObjectError>) -> Self {
        match e {
            SdkError::ConstructionFailure(_)
                => WebErr::Internal(format!("S3 put object failed with construction error")),
            SdkError::TimeoutError(_)
                => WebErr::Internal(format!("S3 put object failed with timeout error")),
            SdkError::DispatchFailure(_)
                => WebErr::Internal(format!("S3 put object failed with dispatch error")),
            SdkError::ResponseError(_)
                => WebErr::Internal(format!("S3 put object failed with response error")),
            SdkError::ServiceError(_)
                => WebErr::Internal(format!("S3 put object failed with service error")),
            _ => WebErr::Internal(format!("S3 put object failed with unknown error")),
        }
    }
}

impl From<std::io::Error> for WebErr {
    fn from(e: std::io::Error) -> Self {
        WebErr::Internal(format!("unexpected IO error: {}", e))
    }
}
