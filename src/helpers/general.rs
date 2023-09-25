use parking_lot::Mutex;
use std::time::SystemTime;
use actix_session::Session;
use actix_web::web;
use prisma_client_rust::or;
use nanoid::nanoid;

use crate::common::WebErr;
use crate::models::events::{LobbyEvent, AllLobbiesEvent, LobbyEventType, Visibility, ChatAlertEvent};
use crate::prisma::{user, PrismaClient, message, game};
use crate::sse::Broadcaster;
use super::game::LobbyVec;


pub fn get_username(session: &Session) -> Result<String, WebErr> {
    match session.get("username") {
        Ok(u) => Ok(u.ok_or(WebErr::Unauth(format!("missing session cookie to get username")))?),
        Err(_) => Err(WebErr::Unauth(format!("missing session cookie to get username"))),
    }
}

pub async fn get_game_by_id(client: &web::Data<PrismaClient>, id: &str) -> Result<game::Data, WebErr> {
    match client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching game with id {}", id))))?
    {
        Some(g) => Ok(g),
        None => Err(WebErr::NotFound(format!("could not find game with id {}", id))),
    }
}

// same as get_game_by_id but fetches user and chat relations
pub async fn get_game_with_relations(client: &web::Data<PrismaClient>, id: &str) -> Result<game::Data, WebErr> {
    client
        .game()
        .find_unique(game::id::equals(id.to_string()))
        .with(game::first_user::fetch().with(user::perfs::fetch(vec![])))
        .with(game::second_user::fetch().with(user::perfs::fetch(vec![])))
        .with(game::chat::fetch(vec![message::game_id::equals(id.to_string())]))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching game with id {}", id))))?
        .ok_or(WebErr::NotFound(format!("could not find game with id {}", id)))
}

pub async fn get_unmatched_games(client: &web::Data<PrismaClient>) -> Result<Vec<game::Data>, WebErr> {
    Ok(client
        .game()
        .find_many(vec![or![
            game::first_username::equals(None),
            game::second_username::equals(None),
        ]])
        .with(game::first_user::fetch().with(user::perfs::fetch(vec![])))
        .with(game::second_user::fetch().with(user::perfs::fetch(vec![])))
        .exec()
        .await
        .or(Err(WebErr::NotFound(format!("error getting all unmatched games"))))?)
}

pub async fn send_lobby_event(client: &web::Data<PrismaClient>, broadcaster: &web::Data<Mutex<Broadcaster>>) -> Result<(), WebErr> {
    let unmatched_games = get_unmatched_games(&client).await?;
    broadcaster.lock().lobby_send(LobbyEvent::AllLobbiesEvent(AllLobbiesEvent {
        r#type: LobbyEventType::AllLobbies,
        lobbies: unmatched_games.to_lobby_vec()?,
    }));
    Ok(())
}

pub async fn get_user_with_relations(client: &web::Data<PrismaClient>, username: &str) -> Result<user::Data, WebErr> {
    client
        .user()
        .find_unique(user::username::equals(username.to_string()))
        .with(user::perfs::fetch(vec![]))
        .with(user::first_user_games::fetch(vec![])
            .with(game::first_user::fetch().with(user::perfs::fetch(vec![])))
            .with(game::second_user::fetch().with(user::perfs::fetch(vec![])))
        )
        .with(user::second_user_games::fetch(vec![])
            .with(game::first_user::fetch().with(user::perfs::fetch(vec![])))
            .with(game::second_user::fetch().with(user::perfs::fetch(vec![])))
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching user {}", username))))?
        .ok_or(WebErr::NotFound(format!("could not find user {}", username)))
}

pub async fn set_user_playing(client: &web::Data<PrismaClient>, username: &str, playing: Option<String>) -> Result<(), WebErr> {
    client
        .user()
        .update(
            user::username::equals(username.to_string()),
            vec![
                user::playing::set(playing),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error setting 'playing' field on user {}", username))))?;
    Ok(())
}

pub async fn add_chat_alert_event(client: &web::Data<PrismaClient>, game_id: &str, event: &ChatAlertEvent) -> Result<(), WebErr> {
    client
        .message()
        .create(
            game::id::equals(game_id.to_string()),
            "".to_string(),
            event.message.clone(),
            Visibility::Player.to_string(),
            true,
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error adding new chat message in game with id {}", game_id))))?;
    Ok(())
}

pub async fn gen_nanoid(client: &web::Data<PrismaClient>) -> String {
    let alphabet: [char; 62] = [
        '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let mut id: String;
    loop {
        id = nanoid!{6, &alphabet};
        if client
            .game()
            .find_unique(game::id::equals(id.clone()))
            .exec()
            .await
            .unwrap()
            .is_none()
        {
            break;
        }
    }
    id
}

pub fn time_millis() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
    as i64
}
