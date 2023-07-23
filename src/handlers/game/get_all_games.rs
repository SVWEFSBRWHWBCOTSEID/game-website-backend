use actix_web::{web::Data, get, HttpResponse};

use crate::prisma::PrismaClient;
use crate::common::WebErr;
use crate::helpers::game::GameVec;


// route for getting all games
#[get("/api/games")]
pub async fn get_all_games(
    client: Data<PrismaClient>,
) -> Result<HttpResponse, WebErr> {

    let games = client
        .game()
        .find_many(vec![])
        .exec()
        .await
        .or(Err(WebErr::Internal(format!("error fetching all games"))))?;

    Ok(HttpResponse::Ok().json(games.to_game_res_vec()?))
}
