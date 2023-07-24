use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::WebErr;
use crate::sse::Broadcaster;


// route for fetching lobbies event stream
#[get("/api/lobbies/events")]
pub async fn new_lobby_client(broadcaster: Data<Mutex<Broadcaster>>) -> Result<HttpResponse, WebErr> {

    let (rx, _) = broadcaster.lock()
        .or(Err(WebErr::Internal(format!("poisoned mutex"))))?
        .new_lobby_client();

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
