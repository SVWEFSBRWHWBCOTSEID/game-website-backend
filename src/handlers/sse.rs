use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::CustomError;
use crate::sse::Broadcaster;


// route for fetching event stream
#[get("/api/events")]
pub async fn new_client(
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {
    let rx = broadcaster.lock().unwrap().new_client();

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
