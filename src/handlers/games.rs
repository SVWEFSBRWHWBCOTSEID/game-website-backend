use actix_web::{web, Error, HttpRequest, HttpResponse};

use crate::models::models::{Game, Seek};


// route for creating a new game
pub async fn create_game(req: HttpRequest, data: web::Json<Seek>) -> Result<HttpResponse, Error> {
    let seek: Seek = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    Ok(HttpResponse::Ok().json(Game::from_seek(seek, game_key)))
}

