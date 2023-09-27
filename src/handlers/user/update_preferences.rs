use parking_lot::Mutex;
use actix_session::Session;
use actix_web::{post, HttpResponse};
use actix_web::web::{Json, Data};

use crate::helpers::general::{get_username};
use crate::models::general::{Preferences};
use crate::prisma::{PrismaClient, preferences};
use crate::sse::Broadcaster;
use crate::common::WebErr;
use crate::models::res::OK_RES;
use crate::models::events::{UserEvent, PreferencesUpdateEvent, UserEventType};


// route for updating a user's preferences
#[post("/api/preferences/update")]
pub async fn update_preferences(
    client: Data<PrismaClient>,
    session: Session,
    data: Json<Preferences>,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let new_preferences = data.into_inner();

    // TODO: create if missing?
    client
        .preferences()
        .update(
            preferences::username::equals(username.clone()),
            vec![
                preferences::show_tenth_seconds::set(new_preferences.clock.show_tenth_seconds.to_string()),
                preferences::show_progress_bars::set(new_preferences.clock.show_progress_bars),
                preferences::play_critical_sound::set(new_preferences.clock.play_critical_sound),
                preferences::confirm_resign::set(new_preferences.game.confirm_resign),
                preferences::board_scroll::set(new_preferences.game.board_scroll),
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Forbidden(format!("could not find preferences for username {}", username))))?;

    broadcaster.lock().user_send(&username, UserEvent::PreferencesUpdateEvent(PreferencesUpdateEvent {
        r#type: UserEventType::PreferencesUpdate,
        preferences: new_preferences
    }));

    Ok(HttpResponse::Ok().json(OK_RES))
}
