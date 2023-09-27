use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, set_user_playing, add_chat_alert_event, get_game_with_relations};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent, ChatAlertEvent};
use crate::models::general::{EndType, Offer};
use crate::player_stats::PlayerStats;
use crate::prisma::{PrismaClient, game};
use crate::common::WebErr;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for telling backend that you have run out of time
#[post("/api/game/{id}/timeout")]
pub async fn timeout(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?.validate(&username)?;
    match (game.get_new_first_time()?, game.get_new_second_time()?) {
        (Some(f), Some(s)) => if f > 0 && s > 0 {
            return Err(WebErr::Forbidden(format!("neither player has timed out on server")))
        },
        _ => return Err(WebErr::Forbidden(format!("cannot time out in untimed game"))),
    }

    let rating_diffs = game.get_rating_diffs(game.get_timeout_game_status(&username)?)?;

    broadcaster.lock().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time()?,
        stime: game.get_new_second_time()?,
        moves: vec![],
        status: game.get_timeout_game_status(&username)?,
        end_type: Some(EndType::Timeout),
        draw_offer: Offer::None,
        frating_diff: rating_diffs.0,
        srating_diff: rating_diffs.1,
    }));

    let chat_alert_event = ChatAlertEvent {
        r#type: GameEventType::ChatAlert,
        message: format!("{} ran out of time", username),
    };
    add_chat_alert_event(&client, &game_id, &chat_alert_event).await?;
    broadcaster.lock().game_send(&game_id, GameEvent::ChatAlertEvent(chat_alert_event));

    set_user_playing(&client, &game.first_username.clone().unwrap(), None).await?;
    set_user_playing(&client, &game.second_username.clone().unwrap(), None).await?;
    game.update_ratings(&client, game.get_timeout_game_status(&username)?).await?;

    player_stats.lock().update_games(-1, &broadcaster.lock());

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(game.get_timeout_game_status(&username)?.to_string()),
                game::win_type::set(Some(EndType::Timeout.to_string())),
                game::draw_offer::set(Offer::None.to_string()),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {} to time out", game_id))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
