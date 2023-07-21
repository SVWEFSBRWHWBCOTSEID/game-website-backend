use std::sync::Mutex;
use actix_session::Session;
use actix_web::{post, HttpRequest, web::{Data, Json}, HttpResponse};

use crate::{prisma::{PrismaClient, game}, helpers::general::get_username};
use crate::models::req::ChatMessageReq;
use crate::models::events::{GameEventType, Visibility, GameEvent, ChatMessageEvent};
use crate::models::res::OK_RES;
use crate::sse::Broadcaster;
use crate::common::WebErr;


// route for sending chat message in a game
#[post("/api/game/{id}/chat/{visibility}")]
pub async fn send_chat(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<ChatMessageReq>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let chat_message_req = data.into_inner();
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let visibility: String = req.match_info().get("visibility").unwrap().parse().unwrap();

    client
        .message()
        .create(
            game::id::equals(game_id.clone()),
            username.clone(),
            chat_message_req.message.clone(),
            visibility.clone(),
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!(""))))?;

    broadcaster.lock().unwrap().game_send(&game_id, GameEvent::ChatMessageEvent(ChatMessageEvent {
        r#type: GameEventType::ChatMessage,
        text: chat_message_req.message,
        username,
        visibility: Visibility::from_str(&visibility)?,
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
