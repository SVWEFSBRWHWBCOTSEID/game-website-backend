use actix_web::{Error, HttpRequest, HttpResponse};

use crate::models::Move;


// route for adding a move to a game
pub async fn add_move(req: HttpRequest) -> Result<HttpResponse, Error> {
    let game_id: String = req.match_info().get("game_id").unwrap().parse().unwrap();
    let user_move: String = req.match_info().query("move").parse().unwrap();
    Ok(HttpResponse::Ok().finish())
}

