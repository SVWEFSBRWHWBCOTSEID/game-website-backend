use std::env;
use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::{get_user_with_relations, get_username, send_lobby_event, set_user_playing};
use crate::models::events::{UserEvent, GameStartEvent, UserEventType};
use crate::models::general::{GameStatus, GameKey};
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, game, user};
use crate::sse::Broadcaster;


// route for joining a game by id
#[post("/api/game/join/{id}")]
pub async fn join_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();

    let game = client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching game with id {}", game_id))))?
        .ok_or(WebErr::NotFound(format!("could not find game with id {}", game_id)))?;

    let user = get_user_with_relations(&client, &username.clone()).await?;
    let perf = user.perfs.as_ref().unwrap().iter().find(|p| p.game_key == game.game_key).unwrap();

    let updated_game = client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            if game.first_username.is_none() {
                vec![
                    game::status::set(GameStatus::Started.to_string()), // TODO: dedupe?
                    game::first_user::connect(user::username::equals(username)),
                    game::first_rating::set(Some(perf.rating as i32)),
                    game::first_prov::set(Some(perf.prov)),
                ]
            } else {
                vec![
                    game::status::set(GameStatus::Started.to_string()), // TODO: dedupe?
                    game::second_user::connect(user::username::equals(username)),
                    game::second_rating::set(Some(perf.rating as i32)),
                    game::second_prov::set(Some(perf.prov)),
                ]
            },
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {}", game_id))))?;

    broadcaster.lock().user_send(&updated_game.first_username.clone().unwrap(), UserEvent::GameStartEvent(GameStartEvent {
        r#type: UserEventType::GameStart,
        game: GameKey::from_str(&updated_game.game_key)?,
        id: game.id.clone(),
    }));
    broadcaster.lock().user_send(&updated_game.second_username.clone().unwrap(), UserEvent::GameStartEvent(GameStartEvent {
        r#type: UserEventType::GameStart,
        game: GameKey::from_str(&updated_game.game_key)?,
        id: game.id.clone(),
    }));

    set_user_playing(&client, &updated_game.first_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat())).await?;
    set_user_playing(&client, &updated_game.second_username.clone().unwrap(), Some([env::var("DOMAIN").unwrap(), "/game/".to_string(), game.id.clone()].concat())).await?;
    send_lobby_event(&client, &broadcaster).await?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
