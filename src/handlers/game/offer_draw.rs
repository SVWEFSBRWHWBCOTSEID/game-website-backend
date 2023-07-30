use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, get_game_validate, set_user_playing, add_chat_alert_event};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent, ChatAlertEvent};
use crate::models::general::Offer;
use crate::prisma::{PrismaClient, game};
use crate::common::WebErr;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/draw/{value}")]
pub async fn offer_draw(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();
    let game = get_game_validate(&client, &game_id, &username).await?;

    broadcaster.lock().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time()?,
        stime: game.get_new_second_time()?,
        moves: vec![],
        status: game.get_draw_game_status(&value, &username)?,
        end_type: None,
        draw_offer: game.get_new_draw_offer(&value, &username)?,
        frating_diff: game.get_rating_diffs(game.get_draw_game_status(&value, &username)?)?.0,
        srating_diff: game.get_rating_diffs(game.get_draw_game_status(&value, &username)?)?.1,
    }));

    let chat_alert_event = ChatAlertEvent {
        r#type: GameEventType::ChatAlert,
        message: match game.get_new_draw_offer(&value, &username)? {
            Offer::None => format!("{} declined the draw offer", username),
            Offer::First | Offer::Second => format!("{} offered a draw", username),
            Offer::Agreed => format!("{} accepted the draw offer", username),
        },
    };
    add_chat_alert_event(&client, &game_id, &chat_alert_event).await?;
    broadcaster.lock().game_send(&game_id, GameEvent::ChatAlertEvent(chat_alert_event));

    if game.get_new_draw_offer(&value, &username)? == Offer::Agreed {
        set_user_playing(&client, &game.first_username.clone().unwrap(), None).await?;
        set_user_playing(&client, &game.second_username.clone().unwrap(), None).await?;
        game.update_ratings(&client, game.get_draw_game_status(&value, &username)?).await?;
    }

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(game.get_draw_game_status(&value, &username)?.to_string()),
                game::draw_offer::set(game.get_new_draw_offer(&value, &username)?.to_string()),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {} to offer draw", game_id))))?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
