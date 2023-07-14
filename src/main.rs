use std::env;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite, time::Duration};
use actix_web::{middleware, web, App, HttpServer};

use game_backend::app_config::config_app;
use game_backend::prisma::PrismaClient;
use game_backend::sse::Broadcaster;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let cookie_domain = env::var("COOKIE_DOMAIN").unwrap();
    
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());
    let redis_store = RedisSessionStore::new(env::var("REDIS_URL").unwrap()).await.unwrap();
    let broadcaster = Broadcaster::create();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at {}:{}", host, port);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(client.clone())
            .app_data(broadcaster.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    redis_store.clone(),
                    Key::from(&[0; 64]),
                )
                .session_lifecycle(
                    PersistentSession::default().session_ttl(Duration::days(3))
                )
                .cookie_domain(match cookie_domain.as_str() {
                    s if s != "" => Some(s.to_string()),
                    _ => None,
                })
                .cookie_same_site(SameSite::None)
                .build()
            )
            .wrap(cors)
            .configure(config_app)
    })
    .bind((host, port))?
    .run()
    .await
}
