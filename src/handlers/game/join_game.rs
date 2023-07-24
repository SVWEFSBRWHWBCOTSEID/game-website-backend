use std::sync::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::{get_username, send_lobby_event};
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

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                if game.first_username.is_none() {
                    game::first_user::connect(user::username::equals(username))
                } else {
                    game::second_user::connect(user::username::equals(username))
                }
            ],
        )
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error updating game with id {}", game_id))))?;

    send_lobby_event(&client, &broadcaster).await?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
