use actix_web::{web, HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::CustomError;
use crate::prisma::{PrismaClient, Side};
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/{game}/game/new")]
pub async fn create_game(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    data: web::Json<CreateGameReq>
) -> Result<HttpResponse, CustomError> {

    let create_game_req: CreateGameReq = data.into_inner();
    if !create_game_req.validate() {
        return Err(CustomError::BadRequest);
    }

    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    let mut rng = rand::thread_rng();
    let playing_first = match create_game_req.side {
        Side::First => true,
        Side::Second => false,
        Side::Random => rng.gen_range(0..1) == 0,
    };

    let game_option = create_game_req.match_if_possible(
        client.clone(),
        &game_key,
        create_game_req.rating_min,
        create_game_req.rating_max,
        playing_first,
    ).await;

    let game = match game_option {
        Some(g) => g,
        None => create_game_req.create_game(
            client,
            &game_key,
            playing_first,
        ).await,
    };

    Ok(HttpResponse::Ok().json(game.to_game_res()))
}
