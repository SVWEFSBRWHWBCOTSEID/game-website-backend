use actix_web::{web::Data, get, HttpResponse};

use crate::prisma::PrismaClient;
use crate::common::WebErr;
use crate::helpers::game::LobbyVec;


// route for getting all games
#[get("/api/lobbies")]
pub async fn get_lobbies(
    client: Data<PrismaClient>,
) -> Result<HttpResponse, WebErr> {

    let games = client
        .game()
        .find_many(vec![])
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching all games"))))?;

    Ok(HttpResponse::Ok().json(games.to_lobby_vec()?))
}
