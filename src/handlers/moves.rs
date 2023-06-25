use actix_web::{Error, HttpRequest, HttpResponse};
use serde::Serialize;


#[derive(Serialize)]
struct TestResponse {
    game_id: String,
    r#move: String,
}

pub async fn test(req: HttpRequest) -> Result<HttpResponse, Error> {
    let game_id: String = req.match_info().get("game_id").unwrap().parse().unwrap();
    let r#move: String = req.match_info().query("move").parse().unwrap();
    let obj = TestResponse {
        game_id,
        r#move,
    };
    Ok(HttpResponse::Ok().json(obj))
}

