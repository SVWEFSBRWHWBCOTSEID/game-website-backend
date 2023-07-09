use std::sync::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, get_game_by_id_validate};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent};
use crate::models::general::{WinType, DrawOffer};
use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/timeout")]
pub async fn timeout(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match get_username(&session) {
        Some(u) => u,
        None => return Err(CustomError::Unauthorized),
    };
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = match get_game_by_id_validate(&client, &game_id, &username).await {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time(),
        stime: game.get_new_second_time(),
        moves: if game.moves.len() > 0 {
            game.moves.split(" ").map(|s| s.to_string()).collect()
        } else {
            vec![]
        },
        status: game.get_timeout_game_status(&username),
        win_type: Some(WinType::Timeout),
        draw_offer: DrawOffer::None,
    }));

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(game.get_timeout_game_status(&username).to_string()),
                game::win_type::set(Some(WinType::Timeout.to_string())),
                game::draw_offer::set(DrawOffer::None.to_bool()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().json(OK_RES))
}
