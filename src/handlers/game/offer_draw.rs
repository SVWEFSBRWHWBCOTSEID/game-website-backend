use std::cmp::max;
use std::sync::Mutex;
use std::time::SystemTime;

use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::models::events::{GameEventType, GameStateEvent, GameEvent};
use crate::models::general::{DrawOffer, WinType};
use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::{general::GameStatus, res::OK_RES};
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/draw/{value}")]
pub async fn offer_draw(
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
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();

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

    let new_game_status = match (
        game.first_username.clone().unwrap() == username,
        value,
        game.draw_offer,
    ) {
        (true, true, Some(false)) => GameStatus::Draw,
        (false, true, Some(true)) => GameStatus::Draw,
        (true, false, Some(false)) => GameStatus::Started,
        (false, false, Some(true)) => GameStatus::Started,
        _ => GameStatus::from_str(&game.status),
    };

    let new_draw_offer = match (
        game.first_username.clone().unwrap() == username,
        value,
        game.draw_offer,
    ) {
        (true, true, None) => DrawOffer::First,
        (false, true, None) => DrawOffer::Second,
        (true, true, Some(false)) => DrawOffer::None,
        (false, true, Some(true)) => DrawOffer::None,
        (true, false, Some(false)) => DrawOffer::None,
        (false, false, Some(true)) => DrawOffer::None,
        _ => DrawOffer::from_bool(&game.draw_offer),
    };

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(new_game_status.to_string()),
                game::draw_offer::set(new_draw_offer.to_bool()),
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
        win_type: match game.win_type {
            Some(wt) => Some(WinType::from_str(&wt)),
            None => None,
        },
        draw_offer: new_draw_offer,
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
