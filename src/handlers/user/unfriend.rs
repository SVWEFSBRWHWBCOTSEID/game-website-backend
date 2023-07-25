use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, HttpRequest, post};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::events::{UserEvent, FriendEvent, UserEventType};
use crate::models::general::FriendRequest;
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, friend};
use crate::sse::Broadcaster;


// route for creating a new user
#[post("/api/unfriend/{username}")]
pub async fn unfriend(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let other_name: String = req.match_info().get("username").unwrap().parse().unwrap();

    if client
        .friend()
        .delete(friend::username_friend_name(username.clone(), other_name.clone()))
        .exec()
        .await
        .is_err()
    {
        client
            .friend()
            .delete(friend::username_friend_name(other_name.clone(), username.clone()))
            .exec()
            .await
            .or(Err(WebErr::Internal(format!("error deleting friend relation from {} to {}", username, other_name))))?;
    }
    broadcaster.lock().user_send(&other_name, UserEvent::FriendEvent(FriendEvent {
        r#type: UserEventType::Friend,
        username,
        value: FriendRequest::Removed,
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
