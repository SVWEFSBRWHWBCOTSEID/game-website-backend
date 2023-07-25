use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use log::info;

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::sse::Broadcaster;


// route for fetching user event stream
#[get("/api/events")]
pub async fn new_user_client(
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;

    info!("locking broadcaster in new_user_client");
    let (rx, _) = broadcaster.lock().new_user_client(username);

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
