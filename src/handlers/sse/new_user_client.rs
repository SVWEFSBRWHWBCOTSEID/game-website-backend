use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::player_stats::PlayerStats;
use crate::sse::Broadcaster;


// route for fetching user event stream
#[get("/api/events")]
pub async fn new_user_client(
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let (rx, _) = broadcaster.lock().new_user_client(username, &player_stats);

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
