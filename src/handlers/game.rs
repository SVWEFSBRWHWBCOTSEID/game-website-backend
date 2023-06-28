use actix_web::{web, Error, HttpRequest, HttpResponse, post};

use crate::prisma::PrismaClient;
use crate::models::game::{Game, Seek};


// route for creating a new game
#[post("/api/{game}/game/new")]
pub async fn create_game(
    req: HttpRequest,
    client: web::Data<PrismaClient>,
    data: web::Json<Seek>
) -> Result<HttpResponse, Error> {

    let seek: Seek = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    let game = Game::from_seek(seek, game_key);

    client
        .game()
        .create(
            game.rated,
            game.game.key,
            game.game.name,
            game.clock.initial,
            game.clock.increment,
            game.start_pos,
            game.state.first_time,
            game.state.second_time,
            game.state.status,
            vec![],
        ).exec()
        .await;

    Ok(HttpResponse::Ok().json(game))
}
