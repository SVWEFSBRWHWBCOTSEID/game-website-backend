use std::sync::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, get_game_by_id_validate};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent};
use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/draw/{value}")]
pub async fn offer_draw(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();
    let game = get_game_by_id_validate(&client, &game_id, &username).await?;

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time(),
        stime: game.get_new_second_time(),
        moves: game.get_moves_vec(),
        status: game.get_draw_game_status(&value, &username),
        win_type: None,
        draw_offer: game.get_new_draw_offer(&value, &username),
    }));

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(game.get_draw_game_status(&value, &username).to_string()),
                game::draw_offer::set(game.get_new_draw_offer(&value, &username).to_bool()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
