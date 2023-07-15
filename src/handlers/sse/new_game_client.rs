use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, HttpRequest};

use crate::common::CustomError;
use crate::helpers::general::get_game_by_id_with_relations;
use crate::models::events::GameEvent;
use crate::models::general::GameStatus;
use crate::prisma::PrismaClient;
use crate::sse::Broadcaster;


// route for fetching game-specific event stream
#[get("/api/game/{id}/events")]
pub async fn new_game_client(
    req: HttpRequest,
    client: Data<PrismaClient>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let rx = broadcaster.lock().unwrap().new_game_client(game_id.clone());
    let game = get_game_by_id_with_relations(&client, &game_id).await?;
    if GameStatus::from_str(&game.status) == GameStatus::Waiting {
        return Err(CustomError::Forbidden);
    }

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameFullEvent(game.to_game_full_event()?));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
