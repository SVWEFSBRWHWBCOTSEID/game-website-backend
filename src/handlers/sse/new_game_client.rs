use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, HttpRequest};

use crate::common::CustomError;
use crate::helpers::general::get_game_by_id_with_relations;
use crate::models::events::{GameEvent, GameFullEvent, GameEventType, ChatMessage, Visibility, GameState};
use crate::models::general::{TimeControl, Player, GameStatus, GameType, WinType, DrawOffer, GameKey};
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

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::GameFullEvent(GameFullEvent {
        r#type: GameEventType::GameFull,
        rated: game.rated,
        game: GameType {
            key: game.game_key.clone(),
            name: GameKey::get_game_name(&game.game_key).expect("invalid game key"),
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
        chat: game.chat.clone().unwrap_or(vec![]).iter().map(|x| ChatMessage {
            username: x.username.clone(),
            text: x.text.clone(),
            visibility: Visibility::from_str(&x.visibility),
        }).collect(),
        state: GameState {
            ftime: game.get_new_first_time(),
            stime: game.get_new_second_time(),
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
