use std::sync::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, get_game_validate, set_user_playing};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent};
use crate::models::general::{WinType, DrawOffer};
use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for telling backend that you have run out of time
#[post("/api/game/{id}/timeout")]
pub async fn timeout(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = get_game_validate(&client, &game_id, &username).await?;
    match (game.get_new_first_time(), game.get_new_second_time()) {
        (Some(f), Some(s)) => if f > 0 && s > 0 {
            return Err(CustomError::Forbidden)
        },
        _ => return Err(CustomError::Forbidden),
    }

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time(),
        stime: game.get_new_second_time(),
        moves: game.get_moves_vec(),
        status: game.get_timeout_game_status(&username),
        win_type: Some(WinType::Timeout),
        draw_offer: DrawOffer::None,
    }));

    set_user_playing(&client, &game.first_username.clone().unwrap(), None).await?;
    set_user_playing(&client, &game.second_username.clone().unwrap(), None).await?;

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
        .or(Err(CustomError::InternalError))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
