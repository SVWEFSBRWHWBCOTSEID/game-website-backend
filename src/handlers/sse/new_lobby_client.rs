use parking_lot::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use log::info;

use crate::common::WebErr;
use crate::helpers::game::LobbyVec;
use crate::helpers::general::get_unmatched_games;
use crate::models::events::{Event, LobbyEvent, AllLobbiesEvent, LobbyEventType};
use crate::prisma::PrismaClient;
use crate::sse::Broadcaster;


// route for fetching lobbies event stream
#[get("/api/lobbies/events")]
pub async fn new_lobby_client(
    client: Data<PrismaClient>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let (rx, tx) = broadcaster.lock().new_lobby_client();

    info!("locking broadcaster in new_lobby_client");
    broadcaster.lock().send_single(&tx, Event::LobbyEvent(
        LobbyEvent::AllLobbiesEvent(AllLobbiesEvent {
            r#type: LobbyEventType::AllLobbies,
            lobbies: get_unmatched_games(&client).await?.to_lobby_vec()?,
        })
    ));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
