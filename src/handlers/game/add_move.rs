use std::{sync::Mutex, time::SystemTime};
use actix_session::Session;
use actix_web::{post, HttpRequest, web::Data, HttpResponse};

use crate::prisma::{PrismaClient, game};
use crate::sse::Broadcaster;
use crate::common::CustomError;
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

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let new_move: String = req.match_info().get("move").unwrap().parse().unwrap();

    let game = match client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    let moves_len = game.moves.split(" ").collect::<Vec<&str>>().len();
    let first_to_move = if game.moves.len() == 0 {
        true
    } else {
        moves_len % 2 == 0
    };

    // respond with 400 if user is not signed in as a player in this game
    if first_to_move && game.first_username.unwrap() != username ||
        !first_to_move && game.second_username.unwrap() != username {

        return Err(CustomError::BadRequest);
    }

    let old_last_move_time = game.last_move_time;
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64;

    let new_first_time = match game.first_time {
        Some(t) => if moves_len >= 2 && moves_len % 2 == 0 {
            Some(t - (current_time - old_last_move_time) as i32)
        } else {
            Some(t)
        },
        None => None,
    };
    let new_second_time = match game.second_time {
        Some(t) => if moves_len >= 2 && moves_len % 2 == 1 {
            Some(t - (current_time - old_last_move_time) as i32)
        } else {
            Some(t)
        },
        None => None,
    };

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::moves::set({
                    let mut moves = game.moves;
                    match moves.len() {
                        0 => moves.push_str(&new_move),
                        _ => {
                            moves.push_str(" ");
                            moves.push_str(&new_move);
                        },
                    }
                    moves
                }),
                game::first_time::set(new_first_time),
                game::second_time::set(new_second_time),
                game::last_move_time::set(current_time),
                game::status::set(GameStatus::Started.to_string()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    broadcaster.lock().unwrap().game_send(game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: new_first_time,
        stime: new_second_time,
        moves: vec![new_move],
        status: GameStatus::from_str(&game.status),
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
