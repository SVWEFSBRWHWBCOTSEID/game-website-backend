use actix_web::web;

use crate::handlers::{games, moves};


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(games::create_game)
        .service(moves::add_move);
}
