use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::{CustomError, get_key_name};
use crate::models::general::{MatchPlayer, Side};
use crate::prisma::{PrismaClient, user, game};
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/{game}/game/new")]
pub async fn create_game(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    session: Session,
    data: web::Json<CreateGameReq>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(u) => u.unwrap(),
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    // respond with 400 if game key is invalid
    if get_key_name(&game_key).is_none() {
        return Err(CustomError::BadRequest);
    }

    let user_option = client
        .user()
        .find_unique(user::username::equals(username))
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
        name: user.username.clone(),
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

    Ok(HttpResponse::Ok().json(game.to_game_res(&client).await))
}

// route for adding a move to a game
#[post("/api/game/{id}/move/{move}")]
pub async fn add_move(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(u) => u.unwrap(),
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let new_move: String = req.match_info().get("move").unwrap().parse().unwrap();

    let game_option = client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .unwrap();

    let game = match game_option {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    // respond with 400 if user is not signed in as a player in this game
    if game.first_username.unwrap() != username && game.second_username.unwrap() != username {
        return Err(CustomError::BadRequest);
    }

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::moves::set({
                    let mut moves = game.moves;
                    moves.push_str(&new_move);
                    moves
                }),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().finish())
}
