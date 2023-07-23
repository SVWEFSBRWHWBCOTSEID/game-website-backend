use actix_web::web;

use crate::handlers::*;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game)
        .service(game::get_game)
        .service(game::get_lobbies)
        .service(game::add_move)
        .service(game::resign)
        .service(game::offer_draw)
        .service(game::send_chat)
        .service(game::timeout)
        .service(user::create_user)
        .service(user::get_user)
        .service(user::get_current_user)
        .service(user::friend_request)
        .service(user::unfriend)
        .service(user::login)
        .service(user::logout)
        .service(sse::new_user_client)
        .service(sse::new_game_client);
}
