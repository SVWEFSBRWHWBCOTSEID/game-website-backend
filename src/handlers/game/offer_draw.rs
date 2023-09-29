use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::helpers::general::{get_username, set_user_playing, add_chat_alert_event, get_game_with_relations, set_user_can_start_game};
use crate::models::events::{GameEventType, GameStateEvent, GameEvent, ChatAlertEvent};
use crate::models::general::Offer;
use crate::player_stats::PlayerStats;
use crate::prisma::{PrismaClient, game};
use crate::common::WebErr;
use crate::lumber_mill::LumberMill;
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;


// route for resigning a game
#[post("/api/game/{id}/draw/{value}")]
pub async fn offer_draw(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
    mill: Data<Mutex<LumberMill>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?.validate(&username)?;

    let rating_diffs = game.get_rating_diffs(game.get_draw_game_status(&value, &username)?)?;

    broadcaster.lock().game_send(&game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.get_new_first_time()?,
        stime: game.get_new_second_time()?,
        moves: vec![],
        status: game.get_draw_game_status(&value, &username)?,
        end_type: None,
        draw_offer: game.get_new_draw_offer(&value, &username)?,
        frating_diff: rating_diffs.0,
        srating_diff: rating_diffs.1,
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
        set_user_can_start_game(&client, &game.first_username.clone().unwrap(), true).await?;
        set_user_can_start_game(&client, &game.second_username.clone().unwrap(), true).await?;
        game.update_ratings(&client, game.get_draw_game_status(&value, &username)?).await?;

        player_stats.lock().update_games(-1, &broadcaster.lock());

        mill.lock().boards.remove(&game.id);
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
