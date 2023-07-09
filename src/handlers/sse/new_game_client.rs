use std::cmp::max;
use std::sync::Mutex;
use std::time::SystemTime;
use actix_web::web::{Data};
use actix_web::{HttpResponse, get, HttpRequest};

use crate::common::{CustomError, get_key_name};
use crate::models::events::{GameEvent, GameFullEvent, GameEventType, ChatMessage, Visibility, GameState};
use crate::models::general::{TimeControl, Player, GameStatus, GameType, WinType, DrawOffer};
use crate::prisma::{PrismaClient, game, message};
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
    
    let game = match client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .with(game::first_user::fetch())
        .with(game::second_user::fetch())
        .with(game::chat::fetch(vec![message::game_id::equals(game_id.clone())]))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    if GameStatus::from_str(&game.status) == GameStatus::Waiting {
        return Err(CustomError::BadRequest);
    }

    let moves_len = game.moves.split(" ").collect::<Vec<&str>>().len();
    let old_last_move_time = game.last_move_time;
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64;

    broadcaster.lock().unwrap().game_send(game_id, GameEvent::GameFullEvent(GameFullEvent {
        r#type: GameEventType::GameFull,
        rated: game.rated,
        game: GameType {
            key: game.game_key.clone(),
            name: get_key_name(&game.game_key).unwrap(),
        },
        time_control: TimeControl {
            initial: game.clock_initial,
            increment: game.clock_increment,
        },
        created_at: game.created_at.to_string(),
        first: Player {
            username: game.first_username.clone().unwrap(),
            provisional: game.first_user().unwrap().unwrap().get_provisional(&game.game_key).unwrap(),
            rating: game.first_user().unwrap().unwrap().get_rating(&game.game_key).unwrap(),
        },
        second: Player {
            username: game.second_username.clone().unwrap(),
            provisional: game.second_user().unwrap().unwrap().get_provisional(&game.game_key).unwrap(),
            rating: game.second_user().unwrap().unwrap().get_rating(&game.game_key).unwrap(),
        },
        chat: game.chat.unwrap_or(vec![]).iter().map(|x| ChatMessage {
            username: x.username.clone(),
            text: x.text.clone(),
            visibility: Visibility::from_str(&x.visibility),
        }).collect(),
        state: GameState {
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
            status: GameStatus::from_str(&game.status),
            win_type: match game.win_type {
                Some(wt) => Some(WinType::from_str(&wt)),
                None => None,
            },
            draw_offer: DrawOffer::from_bool(&game.draw_offer),
        },
    }));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
