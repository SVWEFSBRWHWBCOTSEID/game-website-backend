use actix_web::{web::Data, get, HttpResponse};

use crate::prisma::PrismaClient;
use crate::common::CustomError;
use crate::helpers::game::GameVec;


// route for getting all games
#[get("/api/games")]
pub async fn get_all_games(
    client: Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let games = client
        .game()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)?;

    Ok(HttpResponse::Ok().json(games.to_game_res_vec(&client).await?))
}
