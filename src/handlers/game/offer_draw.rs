use actix_session::Session;
use actix_web::{HttpRequest, post, web::Data, HttpResponse};

use crate::prisma::{PrismaClient, game};
use crate::common::CustomError;
use crate::models::{general::GameStatus, res::OK_RES};


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

    Ok(HttpResponse::Ok().json(OK_RES))
}
