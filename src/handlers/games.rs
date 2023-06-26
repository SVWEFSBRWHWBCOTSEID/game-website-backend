use actix_web::{web, Error, HttpRequest, HttpResponse};
use uuid::Uuid;
use chrono::Utc;

use crate::common::get_key_name;
use crate::models::{Seek, Game, GameType, Clock, Player, GameState, GameStatus};


// route for creating a new game
pub async fn create_game(req: HttpRequest, data: web::Json<Seek>) -> Result<HttpResponse, Error> {
    let seek: Seek = data.into_inner();
    let game_key: String = req.match_info().get("game").unwrap().parse().unwrap();

    let game_type = GameType {
        key: game_key.clone(),
        name: get_key_name(&game_key),
    };
    let clock = Clock {
        initial: seek.time,
        increment: seek.increment,
    };
    let state = GameState {
        moves: Vec::new(),
        first_time: seek.time,
        second_time: seek.time,
        status: GameStatus::Started,
    };
    // for testing
    let second = Player {
        id: Uuid::new_v4(),
        name: "test".to_string(),
        provisional: false,
        rating: 1400,
    };
    let game = Game {
        id: Uuid::new_v4(),
        rated: seek.rated,
        game: game_type,
        clock,
        created_at: Utc::now().timestamp_millis(),
        first: seek.player,
        second,
        start_pos: seek.start_pos,
        state,
    };
    Ok(HttpResponse::Ok().json(game))
}

