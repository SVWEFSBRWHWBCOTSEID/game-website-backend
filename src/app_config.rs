use actix_web::web;

use crate::handlers::moves;


pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/ttt/game")
        .service(web::scope("/{game_id}")
            .service(web::scope("/move")
                .service(web::resource("/{move}")
                    .route(web::get().to(moves::test))
                )
            )
        )
    );
}

