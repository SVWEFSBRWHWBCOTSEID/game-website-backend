use actix_web::web;

use crate::handlers::{game, user};


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game)
        .service(game::add_move)
        .service(user::create_user)
        .service(user::get_user)
        .service(user::login)
        .service(user::profile);
}
