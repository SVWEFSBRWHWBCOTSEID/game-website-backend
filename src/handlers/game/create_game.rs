use parking_lot::Mutex;
use actix_session::Session;
use actix_web::web::{Json, Data};
use actix_web::{HttpRequest, HttpResponse, post};

use crate::common::WebErr;
use crate::helpers::general::{get_username, get_user_with_relations, get_game_with_relations};
use crate::lumber_mill::LumberMill;
use crate::player_stats::PlayerStats;
use crate::prisma::PrismaClient;
use crate::models::req::CreateGameReq;
use crate::sse::Broadcaster;


// route for creating a new game
#[post("/api/game/new/{game}")]
pub async fn create_game(
    req: HttpRequest,
    client: Data<PrismaClient>,
    session: Session,
    data: Json<CreateGameReq>,
    broadcaster: Data<Mutex<Broadcaster>>,
    player_stats: Data<Mutex<PlayerStats>>,
    mill: Data<Mutex<LumberMill>>,
) -> Result<HttpResponse, WebErr> {

    let username: String = get_username(&session)?;
    let create_game_req: CreateGameReq = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    let match_player = get_user_with_relations(&client, &username)
        .await?
        .to_match_player(&game_key, &create_game_req);
    let game = create_game_req.create_or_join(&client, &game_key, &match_player, &broadcaster, &player_stats).await?;

    mill.lock().create_board_from_game(&game)?;

    let game_with_relations = get_game_with_relations(&client, &game.id).await?;

    Ok(HttpResponse::Ok().json(game_with_relations.to_create_game_res(&client).await?))
}
