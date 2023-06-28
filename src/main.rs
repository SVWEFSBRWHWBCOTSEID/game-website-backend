use actix_web::{middleware, App, HttpServer};

use game_backend::app_config::config_app;
use game_backend::prisma::PrismaClient;
use prisma_client_rust::NewClientError;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = wb::Data::new(PrismaClient::_builder().build().await.unwrap());

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .app_data(client.clone())
            .wrap(middleware::Logger::default())
            .configure(config_app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
