use actix_web::web;

use crate::handlers::*;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game)
        .service(game::get_game)
        .service(game::add_move)
        .service(game::resign)
        .service(game::offer_draw)
        .service(game::send_chat)
        .service(user::create_user)
        .service(user::get_user)
        .service(user::login)
        .service(user::logout)
        .service(sse::new_user_client)
        .service(sse::new_game_client);
}
