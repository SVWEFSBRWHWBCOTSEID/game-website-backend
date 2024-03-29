use parking_lot::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::game::LobbyVec;
use crate::helpers::general::get_unmatched_games;
use crate::models::events::{Event, LobbyEvent, LobbyEventType, LobbyFullEvent};
use crate::player_stats::PlayerStats;
use crate::prisma::PrismaClient;
use crate::sse::Broadcaster;


// route for fetching lobbies event stream
#[get("/api/lobbies/events")]
pub async fn new_lobby_client(
    client: Data<PrismaClient>,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
) -> Result<HttpResponse, WebErr> {

    let (rx, tx) = broadcaster.lock().new_lobby_client();
    let unmatched_games = get_unmatched_games(&client).await?;

    let stats = player_stats.lock();

    broadcaster.lock().send_single(&tx, Event::LobbyEvent(
        LobbyEvent::LobbyFullEvent(LobbyFullEvent {
            r#type: LobbyEventType::LobbyFull,
            lobbies: unmatched_games.to_lobby_vec()?,
            players: stats.players,
            games: stats.games,
        })
    ));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
