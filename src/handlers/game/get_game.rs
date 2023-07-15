use actix_web::{HttpRequest, web::Data, get, HttpResponse};

use crate::helpers::general::get_game_by_id;
use crate::prisma::PrismaClient;
use crate::common::CustomError;


// route for getting a game by id
#[get("/api/game/{id}")]
pub async fn get_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = get_game_by_id(&client, &game_id).await?;

    Ok(HttpResponse::Ok().json(game.to_game_res(&client).await?))
}
