use actix_session::Session;
use actix_web::web::{Json, Data};
use actix_web::{HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::common::{CustomError, get_key_name};
use crate::models::general::{MatchPlayer, Side};
use crate::prisma::{PrismaClient, user};
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/game/new/{game}")]
pub async fn create_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<CreateGameReq>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    // respond with 400 if game key is invalid
    if get_key_name(&game_key).is_none() {
        return Err(CustomError::BadRequest);
    }

    let user = match client
        .user()
        .find_unique(user::username::equals(username))
        .exec()
        .await
        .unwrap()
    {
        Some(u) => u,
        None => return Err(CustomError::BadRequest),
    };

    let mut rng = rand::thread_rng();

    let match_player = MatchPlayer {
        username: user.username.clone(),
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
    if !create_game_req.validate(&client, &match_player).await {
        return Err(CustomError::BadRequest);
    }

    // match with an open game if possible, otherwise create new
    let game = match create_game_req.match_if_possible(
        &client,
        &game_key,
        &match_player,
    ).await {
        Some(g) => g,
        None => create_game_req.create_game(
            &client,
            &game_key,
            &match_player,
        ).await,
    };

    Ok(HttpResponse::Ok().json(game.to_create_game_res(&client).await))
}
