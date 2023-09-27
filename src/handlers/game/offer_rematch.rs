use std::env;
use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::{get_username, set_user_playing, gen_nanoid, add_chat_alert_event, get_game_with_relations};
use crate::lumber_mill::LumberMill;
use crate::models::events::{GameEvent, GameEventType, RematchEvent, ChatAlertEvent};
use crate::models::general::{Offer, GameStatus};
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, game, user};
use crate::sse::Broadcaster;


// route for creating a new game
#[post("/api/game/{id}/rematch/{value}")]
pub async fn offer_rematch(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    mill: Data<Mutex<LumberMill>>
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?.validate_ended(&username)?;

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::rematch_offer::set(game.get_new_rematch_offer(&value, &username)?.to_string()),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {} to offer rematch", game_id))))?;

    let new_rematch_offer = game.get_new_rematch_offer(&value, &username)?;
    if new_rematch_offer == Offer::Agreed {
        let id = gen_nanoid(&client).await;

        client
            .game()
            .create(
                id.clone(),
                game.rated,
                game.game_key.clone(),
                game.rating_min,
                game.rating_max,
                "".to_string(),
                0,
                GameStatus::Started.to_string(),
                Offer::None.to_string(),
                Offer::None.to_string(),
                false,
                vec![
                    game::clock_initial::set(game.clock_initial),
                    game::clock_increment::set(game.clock_increment),
                    game::first_time::set(game.clock_initial),
                    game::second_time::set(game.clock_initial),
                    game::first_rating::set(Some(game.second_user().unwrap().unwrap().get_rating(&game.game_key)? as i32)),
                    game::second_rating::set(Some(game.first_user().unwrap().unwrap().get_rating(&game.game_key)? as i32)),
                    game::first_prov::set(Some(game.second_user().unwrap().unwrap().get_provisional(&game.game_key)?)),
                    game::second_prov::set(Some(game.first_user().unwrap().unwrap().get_provisional(&game.game_key)?)),
                    game::start_pos::set(game.start_pos.clone()),
                    game::first_user::connect(user::username::equals(game.second_username.clone().unwrap())),
                    game::second_user::connect(user::username::equals(game.first_username.clone().unwrap())),
                ],
            )
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error creating game"))))?;

        set_user_playing(&client, &game.first_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), id.clone()].concat())).await?;
        set_user_playing(&client, &game.second_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), id.clone()].concat())).await?;

        mill.lock().create_board_from_game(&game)?;

        broadcaster.lock().game_send(&game.id, GameEvent::RematchEvent(RematchEvent {
            r#type: GameEventType::Rematch,
            rematch_offer: Offer::Agreed,
            id: Some(id),
        }));

        let chat_alert_event = ChatAlertEvent {
            r#type: GameEventType::ChatAlert,
            message: format!("{} accepted the rematch", username),
        };
        add_chat_alert_event(&client, &game_id, &chat_alert_event).await?;
        broadcaster.lock().game_send(&game_id, GameEvent::ChatAlertEvent(chat_alert_event));
    } else {
        broadcaster.lock().game_send(&game.id, GameEvent::RematchEvent(RematchEvent {
            r#type: GameEventType::Rematch,
            rematch_offer: new_rematch_offer,
            id: None,
        }));

        let chat_alert_event = ChatAlertEvent {
            r#type: GameEventType::ChatAlert,
            message: format!("{} offered a rematch", username),
        };
        add_chat_alert_event(&client, &game_id, &chat_alert_event).await?;
        broadcaster.lock().game_send(&game_id, GameEvent::ChatAlertEvent(chat_alert_event));
    }

    Ok(HttpResponse::Ok().json(OK_RES))
}
