use actix_web::web;

use crate::handlers::{game, user};


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game)
        .service(user::create_user);
}
