use actix_web::{web, HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::{CustomError, get_key_name};
use crate::models::general::MatchPlayer;
use crate::prisma::{PrismaClient, Side, user};
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/{game}/game/new")]
pub async fn create_game(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    data: web::Json<CreateGameReq>
) -> Result<HttpResponse, CustomError> {

    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    // respond with 400 if game key is invalid
    if get_key_name(&game_key).is_none() {
        return Err(CustomError::BadRequest);
    }

    let user_option = client
        .user()
        .find_unique(user::name::equals(create_game_req.username.clone()))
        .exec()
        .await
        .unwrap();

    let user: user::Data;
    // respond with 400 if user cannot be found
    match user_option {
        Some(u) => user = u,
        None => return Err(CustomError::BadRequest),
    }

    let mut rng = rand::thread_rng();

    let match_player = MatchPlayer {
        name: user.name.clone(),
        provisional: user.get_provisional(&game_key).unwrap(),
        rating: user.get_rating(&game_key).unwrap(),
        rating_min: create_game_req.rating_min,
        rating_max: create_game_req.rating_max,
        first: match create_game_req.side {
            Side::First => true,
            Side::Second => false,
            Side::Random => rng.gen_range(0..1) == 0,
        },
    };

    // respond with 400 if request is invalid
    if !create_game_req.validate(&match_player) {
        return Err(CustomError::BadRequest);
    }

    // see if an open game fits criteria
    let game_option = create_game_req.match_if_possible(
        &client,
        &game_key,
        &match_player,
    ).await;

    // otherwise, create a new game
    let game = match game_option {
        Some(g) => g,
        None => create_game_req.create_game(
            &client,
            &game_key,
            &match_player,
        ).await,
    };

    Ok(HttpResponse::Ok().json(game.to_game_res()))
}
