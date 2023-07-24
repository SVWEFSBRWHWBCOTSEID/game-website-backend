use std::sync::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpResponse, post};
use prisma_client_rust::or;

use crate::common::WebErr;
use crate::helpers::general::{get_username, send_lobby_event};
use crate::models::res::OK_RES;
use crate::prisma::{PrismaClient, game, SortOrder};
use crate::sse::Broadcaster;


// route for canceling a new game
#[post("/api/game/cancel")]
pub async fn cancel_game(
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let game = client
        .game()
        .find_many(vec![or![
            game::first_username::equals(Some(username.clone())),
            game::second_username::equals(Some(username.clone())),
        ]])
        .order_by(game::created_at::order(SortOrder::Desc))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching user {}'s games", username))))?
        .into_iter()
        .next()
        .ok_or(WebErr::Forbidden(format!("no games with user {}", username)))?;

    client
        .game()
        .delete(game::id::equals(game.id.clone()))
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error deleting game with id {}", game.id))))?;

    send_lobby_event(&client, &broadcaster).await?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
