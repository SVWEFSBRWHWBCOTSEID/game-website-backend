use std::sync::Mutex;
use actix_session::Session;
use actix_web::web::{Data};
use actix_web::{HttpResponse, get, HttpRequest};

use crate::common::CustomError;
use crate::models::events::{GameEvent, GameFullEvent, GameEventType, ChatMessage, Visibility, GameState};
use crate::models::general::{TimeControl, Player, GameStatus};
use crate::prisma::{PrismaClient, game};
use crate::sse::Broadcaster;


// route for fetching user event stream
#[get("/api/events")]
pub async fn new_user_client(
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

    let rx = broadcaster.lock().unwrap().new_user_client(username);

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}

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

    broadcaster.lock().unwrap().game_send(game_id, GameEvent::GameFullEvent(GameFullEvent {
        r#type: GameEventType::GameFull,
        rated: game.rated,
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
            ftime: game.first_time,
            stime: game.second_time,
            status: GameStatus::from_str(&game.status),
            moves: if game.moves.len() > 0 {
                game.moves.split(" ").map(|s| s.to_string()).collect()
            } else {
                vec![]
            },
        },
    }));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
