use actix_session::storage::RedisSessionStore;
use actix_web::cookie::Key;
use actix_web::{web, middleware, App, HttpServer};
use actix_cors::Cors;
use actix_session::SessionMiddleware;

use game_backend::app_config::config_app;
use game_backend::prisma::PrismaClient;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    let secret_key = Key::generate();
    let redis_uri = "redis://127.0.0.1:6379";
    let redis_store = RedisSessionStore::new(redis_uri).await.unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(client.clone())
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .wrap(SessionMiddleware::new(redis_store.clone(), secret_key.clone()))
            .configure(config_app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
