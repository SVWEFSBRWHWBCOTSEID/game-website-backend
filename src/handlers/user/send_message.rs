use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::events::{UserEvent, UserEventType, UserMessageEvent};
use crate::models::req::UserMessageReq;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, conversation, user};
use crate::sse::Broadcaster;


// route for sending message
#[post("/api/message/{username}")]
pub async fn send_message(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<UserMessageReq>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let user_message_req = data.into_inner();
    let other_name: String = req.match_info().get("username").unwrap().parse().unwrap();

    let (first_name, second_name) = if username < other_name {
        (username.clone(), other_name.clone())
    } else {
        (other_name.clone(), username.clone())
    };

    let conversation = client
        .conversation()
        .upsert(
            conversation::username_other_name(first_name.clone(), second_name.clone()),
            conversation::create(
                user::username::equals(first_name.clone()),
                second_name.clone(),
                vec![]
            ),
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(
            format!("failed to create or fetch conversation for {} and {}", first_name.clone(), second_name.clone())
        )))?;

    let message = client
        .user_message()
        .create(
            conversation::id::equals(conversation.id),
            username.clone(),
            user_message_req.message,
            vec![],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(
            format!("failed to create new message for conversation with {} and {}", first_name.clone(), second_name.clone())
        )))?;

    broadcaster.lock().user_send(&other_name, UserEvent::UserMessageEvent(UserMessageEvent {
        r#type: UserEventType::UserMessage,
        username,
        text: message.text,
        created_at: message.created_at.to_string(),
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
