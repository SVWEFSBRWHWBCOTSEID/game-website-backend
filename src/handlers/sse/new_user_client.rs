use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

use crate::common::WebErr;
use crate::helpers::general::get_username;
use crate::models::events::{PreferencesUpdateEvent, UserEvent, UserEventType};
use crate::prisma::{preferences, PrismaClient};
use crate::sse::Broadcaster;


// route for fetching user event stream
#[get("/api/events")]
pub async fn new_user_client(
    session: Session,
    client: Data<PrismaClient>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let (rx, _) = broadcaster.lock().new_user_client(username.clone());

    let preferences = client
        .preferences()
        .find_unique(preferences::username::equals(username.to_string()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching preferences for user {}", username))))?
        .ok_or(WebErr::NotFound(format!("could not find preferences for user {}", username)))?;

    broadcaster.lock().user_send(&username, UserEvent::PreferencesUpdateEvent(PreferencesUpdateEvent {
        r#type: UserEventType::PreferencesUpdate,
        preferences: preferences.to_preferences_res()?,
    }));

    Ok(HttpResponse::Ok()
        .append_header(("content-type", "text/event-stream"))
        .streaming(rx)
    )
}
