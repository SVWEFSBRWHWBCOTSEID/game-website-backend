use std::sync::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::CustomError;
use crate::sse::Broadcaster;


// route for fetching user event stream
#[get("/api/events")]
pub async fn new_user_client(
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let rx = broadcaster.lock().unwrap().new_user_client(username);

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
