use actix_web::{HttpRequest, web::Data, get, HttpResponse};

use crate::prisma::PrismaClient;
use crate::common::{CustomError, get_game_by_id};


// route for getting a game by id
#[get("/api/game/{id}")]
pub async fn get_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
) -> Result<HttpResponse, CustomError> {

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let game = match get_game_by_id(&client, &game_id).await {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    Ok(HttpResponse::Ok().json(game.to_game_res(&client).await))
}
