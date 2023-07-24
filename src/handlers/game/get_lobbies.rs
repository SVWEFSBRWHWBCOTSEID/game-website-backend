use actix_web::{web::Data, get, HttpResponse};

use crate::helpers::general::get_unmatched_games;
use crate::prisma::PrismaClient;
use crate::common::WebErr;
use crate::helpers::game::LobbyVec;


// route for getting all games
#[get("/api/lobbies")]
pub async fn get_lobbies(client: Data<PrismaClient>) -> Result<HttpResponse, WebErr> {
    Ok(HttpResponse::Ok().json(get_unmatched_games(&client).await?.to_lobby_vec()?))
}
