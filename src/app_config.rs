use actix_web::web;

use crate::handlers::game;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game);
}
