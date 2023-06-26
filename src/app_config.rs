use actix_web::web;

use crate::handlers::{games, moves};


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .service(web::scope("/{game}")
            .service(web::scope("/game")
                .service(web::resource("/new")
                    .route(web::post().to(games::create_game))
                )
                .service(web::scope("/{game_id}")
                    .service(web::scope("/move")
                        .service(web::resource("/{move}")
                            .route(web::post().to(moves::add_move))
                        )
                    )
                )
            )
        )
    );
}

