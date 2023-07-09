use std::sync::Mutex;
use actix_session::Session;
use actix_web::{post, HttpRequest, web::Data, HttpResponse};

use crate::models::general::{WinType, DrawOffer};
use crate::prisma::{PrismaClient, game};
use crate::sse::Broadcaster;
use crate::common::{CustomError, get_username, get_game_by_id, time_millis};
use crate::models::{general::GameStatus, res::OK_RES};
use crate::models::events::{GameEvent, GameStateEvent, GameEventType};


// route for adding a move to a game
#[post("/api/game/{id}/move/{move}")]
pub async fn add_move(
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
    let new_move: String = req.match_info().get("move").unwrap().parse().unwrap();
    let game = match get_game_by_id(&client, &game_id).await {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    let first_to_move = game.num_moves() % 2 == 0;
    if first_to_move && game.first_username.clone().unwrap() != username ||
        !first_to_move && game.second_username.clone().unwrap() != username {

        return Err(CustomError::BadRequest);
    }

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time(),
        stime: game.get_new_second_time(),
        moves: vec![new_move.clone()],
        status: GameStatus::from_str(&game.status),
        win_type: match game.win_type.clone() {
            Some(wt) => Some(WinType::from_str(&wt)),
            None => None,
        },
        draw_offer: DrawOffer::from_bool(&game.draw_offer),
    }));

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::moves::set({
                    let mut moves = game.moves.clone();
                    match moves.len() {
                        0 => moves.push_str(&new_move),
                        _ => {
                            moves.push_str(" ");
                            moves.push_str(&new_move);
                        },
                    }
                    moves
                }),
                game::first_time::set(game.get_new_first_time()),
                game::second_time::set(game.get_new_second_time()),
                game::last_move_time::set(time_millis()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().json(OK_RES))
}
