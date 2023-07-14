use actix_session::Session;
use actix_web::web::{Json, Data};
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::CustomError;
use crate::helpers::general::{get_username, get_user_by_username};
use crate::models::general::GameKey;
use crate::prisma::PrismaClient;
use crate::models::req::CreateGameReq;


// route for creating a new game
#[post("/api/game/new/{game}")]
pub async fn create_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<CreateGameReq>,
) -> Result<HttpResponse, CustomError> {

    let username: String = get_username(&session)?;
    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();
    if GameKey::get_game_name(&game_key).is_none() {
        return Err(CustomError::BadRequest);
    }

    let match_player = get_user_by_username(&client, &username)
        .await?
        .to_match_player(&game_key, &create_game_req);
    let game = create_game_req.create_or_join(&client, &game_key, &match_player).await?;

    Ok(HttpResponse::Ok().json(game.to_create_game_res(&client).await))
}
