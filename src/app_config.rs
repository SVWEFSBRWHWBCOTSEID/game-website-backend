use actix_web::web;

use crate::handlers::*;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg
        .service(game::create_game)
        .service(game::cancel_game)
        .service(game::join_game)
        .service(game::get_game)
        .service(game::get_lobbies)
        .service(game::add_move)
        .service(game::resign)
        .service(game::offer_draw)
        .service(game::offer_rematch)
        .service(game::send_chat)
        .service(user::create_user)
        .service(user::create_guest)
        .service(user::get_user)
        .service(user::get_current_user)
        .service(user::update_profile)
        .service(user::update_preferences)
        .service(user::friend_request)
        .service(user::unfriend)
        .service(user::send_message)
        .service(user::get_conversations)
        .service(user::challenge_request)
        .service(user::login)
        .service(user::logout)
        .service(sse::new_user_client)
        .service(sse::new_game_client)
        .service(sse::new_lobby_client);
}
