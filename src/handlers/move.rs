use actix_web::{Error, HttpRequest, HttpResponse, post};


// route for adding a move to a game
#[post("/api/{game}/game/{game_id}/move/{move}")]
pub async fn add_move(req: HttpRequest) -> Result<HttpResponse, Error> {
    let _game_key: String = req.match_info().get("game").unwrap().parse().unwrap();
    let _game_id: String = req.match_info().get("game_id").unwrap().parse().unwrap();
    let _user_move: String = req.match_info().query("move").parse().unwrap();
    Ok(HttpResponse::Ok().finish())
}
