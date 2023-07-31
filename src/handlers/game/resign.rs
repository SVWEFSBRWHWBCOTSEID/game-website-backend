use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{post, HttpRequest, web::Data, HttpResponse};

use crate::helpers::general::{get_username, set_user_playing, add_chat_alert_event, get_game_with_relations};
use crate::models::events::{GameEvent, GameStateEvent, GameEventType, ChatAlertEvent};
use crate::models::general::{EndType, Offer};
use crate::prisma::{PrismaClient, game};
use crate::common::WebErr;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/resign")]
pub async fn resign(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?.validate(&username)?;

    let rating_diffs = game.get_rating_diffs(game.get_resign_game_status(&username))?;

    broadcaster.lock().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time()?,
        stime: game.get_new_second_time()?,
        moves: vec![],
        status: game.get_resign_game_status(&username),
        end_type: Some(EndType::Resign),
        draw_offer: Offer::None,
        frating_diff: rating_diffs.0,
        srating_diff: rating_diffs.1,
    }));

    let chat_alert_event = ChatAlertEvent {
        r#type: GameEventType::ChatAlert,
        message: format!("{} resigned", username),
    };
    add_chat_alert_event(&client, &game_id, &chat_alert_event).await?;
    broadcaster.lock().game_send(&game_id, GameEvent::ChatAlertEvent(chat_alert_event));

    set_user_playing(&client, &game.first_username.clone().unwrap(), None).await?;
    set_user_playing(&client, &game.second_username.clone().unwrap(), None).await?;
    game.update_ratings(&client, game.get_resign_game_status(&username)).await?;

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(game.get_resign_game_status(&username).to_string()),
                game::win_type::set(Some(EndType::Resign.to_string())),
                game::draw_offer::set(Offer::None.to_string()),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {} to resign", game_id))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
