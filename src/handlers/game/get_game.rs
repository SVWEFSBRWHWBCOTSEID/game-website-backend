use actix_web::{HttpRequest, web::Data, get, HttpResponse};

use crate::helpers::general::get_game_with_relations;
use crate::prisma::PrismaClient;
use crate::common::WebErr;


// route for getting a game by id
#[get("/api/game/{id}")]
pub async fn get_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
) -> Result<HttpResponse, WebErr> {

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = get_game_with_relations(&client, &game_id).await?;

    Ok(HttpResponse::Ok().json(game.to_game_res(&client).await?))
}
