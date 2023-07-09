use std::cmp::max;
use std::sync::Mutex;
use std::time::SystemTime;

use actix_session::Session;
use actix_web::{post, HttpRequest, web::Data, HttpResponse};

use crate::models::events::{GameEvent, GameStateEvent, GameEventType};
use crate::models::general::{WinType, DrawOffer};
use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::{general::GameStatus, res::OK_RES};
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/resign")]
pub async fn resign(
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

    // respond with 400 if the game is not in progress or if user is not signed in as a player in this game
    if GameStatus::from_str(&game.status) != GameStatus::Started ||
        game.first_username.clone().unwrap() != username && game.second_username.unwrap() != username {
            
        return Err(CustomError::BadRequest);
    }

    let new_game_status = if game.first_username.clone().unwrap() == username {
        GameStatus::SecondWon
    } else {
        GameStatus::FirstWon
    };

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(new_game_status.to_string()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    let moves_len = game.moves.split(" ").collect::<Vec<&str>>().len();
    let old_last_move_time = game.last_move_time;
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64;

    broadcaster.lock().unwrap().game_send(game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: match game.first_time {
            Some(t) => if moves_len >= 2 && moves_len % 2 == 0 {
                Some(max(0, t - (current_time - old_last_move_time) as i32))
            } else {
                Some(t)
            },
            None => None,
        },
        stime: match game.second_time {
            Some(t) => if moves_len >= 2 && moves_len % 2 == 1 {
                Some(max(0, t - (current_time - old_last_move_time) as i32))
            } else {
                Some(t)
            },
            None => None,
        },
        moves: if game.moves.len() > 0 {
            game.moves.split(" ").map(|s| s.to_string()).collect()
        } else {
            vec![]
        },
        status: new_game_status,
        win_type: Some(WinType::Resign),
        draw_offer: DrawOffer::None,
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
