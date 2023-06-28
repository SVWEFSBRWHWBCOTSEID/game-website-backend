use actix_web::{web, Error, HttpRequest, HttpResponse, post};

use crate::prisma::PrismaClient;
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/{game}/game/new")]
pub async fn create_game(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    data: web::Json<CreateGameReq>
) -> Result<HttpResponse, Error> {

    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    let game = create_game_req.create_game(client, &game_key).await;

    Ok(HttpResponse::Ok().json(game.to_game_res()))
}
