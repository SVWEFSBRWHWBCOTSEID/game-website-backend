use std::sync::Mutex;
use actix_session::Session;
use actix_web::{web::{Json, Data}, HttpRequest, HttpResponse, post};
use rand::Rng;

use crate::common::{CustomError, get_key_name};
use crate::models::events::{GameEvent, GameStateEvent, GameEventType};
use crate::models::general::{MatchPlayer, Side, GameStatus};
use crate::prisma::{PrismaClient, user, game};
use crate::models::req::{CreateGameReq, ChatMessageReq};
use crate::sse::Broadcaster;


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
    if !create_game_req.validate(&match_player) {
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

    Ok(HttpResponse::Ok().json(game.to_game_res(&client).await))
}

// route for adding a move to a game
#[post("/api/game/{id}/move/{move}")]
pub async fn add_move(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    broadcaster: Data<Mutex<Broadcaster>>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let new_move: String = req.match_info().get("move").unwrap().parse().unwrap();

    let game = match client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    let first_to_move = game.moves.len() % 2 == 0;

    // respond with 400 if user is not signed in as a player in this game
    if first_to_move && game.first_username.unwrap() != username ||
        !first_to_move && game.second_username.unwrap() != username {

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
                    moves.push_str(" ");
                    moves
                }),
                game::status::set(GameStatus::Started.to_string()),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    broadcaster.lock().unwrap().game_send(game_id, GameEvent::GameStateEvent(GameStateEvent {
        r#type: GameEventType::GameState,
        ftime: game.first_time,
        stime: game.second_time,
        moves: vec![new_move],
        status: GameStatus::from_str(&game.status),
    }));

    Ok(HttpResponse::Ok().finish())
}

// route for resigning a game
#[post("/api/game/{id}/resign")]
pub async fn resign(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();

    let game = match client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    // respond with 400 if the game has not begun yet or if user is not signed in as a player in this game
    if GameStatus::from_str(&game.status) == GameStatus::Waiting ||
        game.first_username.clone().unwrap() != username || game.second_username.unwrap() != username {
            
        return Err(CustomError::BadRequest);
    }

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(if game.first_username.unwrap() == username {
                    GameStatus::FirstResigned.to_string()
                } else {
                    GameStatus::SecondResigned.to_string()
                }),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().finish())
}

// route for resigning a game
#[post("/api/game/{id}/draw/{value}")]
pub async fn offer_draw(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let value: bool = req.match_info().get("value").unwrap().parse().unwrap();

    let game = match client
        .game()
        .find_unique(game::id::equals(game_id.clone()))
        .exec()
        .await
        .unwrap()
    {
        Some(g) => g,
        None => return Err(CustomError::BadRequest),
    };

    // respond with 400 if the game has not begun yet or if user is not signed in as a player in this game
    if GameStatus::from_str(&game.status) == GameStatus::Waiting ||
        game.first_username.clone().unwrap() != username || game.second_username.unwrap() != username {

        return Err(CustomError::BadRequest);
    }

    client
        .game()
        .update(
            game::id::equals(game_id.clone()),
            vec![
                game::status::set(match (
                    game.first_username.unwrap() == username,
                    value,
                    GameStatus::from_str(&game.status),
                ) {
                    (true, true, GameStatus::Started) => GameStatus::FirstDrawOffer.to_string(),
                    (false, true, GameStatus::Started) => GameStatus::SecondDrawOffer.to_string(),
                    (true, true, GameStatus::SecondDrawOffer) => GameStatus::Draw.to_string(),
                    (false, true, GameStatus::FirstDrawOffer) => GameStatus::Draw.to_string(),
                    (true, false, GameStatus::SecondDrawOffer) => GameStatus::Started.to_string(),
                    (false, false, GameStatus::FirstDrawOffer) => GameStatus::Started.to_string(),
                    _ => game.status,
                }),
            ],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().finish())
}

// route for sending chat message in a game
#[post("/api/game/{id}/chat/{visibility}")]
pub async fn send_chat(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<ChatMessageReq>,
) -> Result<HttpResponse, CustomError> {

    let username: String = match session.get("username") {
        Ok(o) => match o {
            Some(u) => u,
            None => return Err(CustomError::Unauthorized),
        },
        Err(_) => return Err(CustomError::Unauthorized),
    };

    let chat_message_req = data.into_inner();
    let game_id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let visibility: String = req.match_info().get("visibility").unwrap().parse().unwrap();

    client
        .message()
        .create(
            game::id::equals(game_id),
            username,
            chat_message_req.message,
            visibility,
            vec![],
        )
        .exec()
        .await
        .map_err(|_| CustomError::InternalError)
        .ok();

    Ok(HttpResponse::Ok().finish())
}
