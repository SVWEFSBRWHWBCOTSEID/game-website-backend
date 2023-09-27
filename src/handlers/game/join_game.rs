use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::{get_user_with_relations, get_username};
use crate::helpers::create_game::join_game as join_game_util;
use crate::models::res::OK_RES;
use crate::player_stats::PlayerStats;
use crate::prisma::{PrismaClient, game};
use crate::sse::Broadcaster;


// route for joining a game by id
#[post("/api/game/join/{id}")]
pub async fn join_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
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
    let perf = user.perfs().unwrap().iter().find(|p| p.game_key == game.game_key).unwrap();

    join_game_util(&client, &game, game.first_user.is_none(), username, perf.rating as i32, perf.prov, &broadcaster, &player_stats).await?;

    Ok(HttpResponse::Ok().json(OK_RES))
}
