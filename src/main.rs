use std::env;
use std::sync::Arc;
use dotenv::dotenv;
use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{Key, SameSite, time::Duration};
use actix_web::{middleware, web, App, HttpServer};
use aws_config::BehaviorVersion;
use aws_config::meta::region::RegionProviderChain;

use game_backend::app_config::config_app;
use game_backend::hourglass::Hourglass;
use game_backend::lumber_mill::LumberMill;
use game_backend::player_stats::PlayerStats;
use game_backend::prisma::PrismaClient;
use game_backend::sse::Broadcaster;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _guard = sentry::init(("https://3f4ae5fdde0f9e4e30745eb533fcaab8@o4505684851294208.ingest.sentry.io/4505684891467776", sentry::ClientOptions {
        release: sentry::release_name!(),
        traces_sample_rate: 1.0,
        before_send: Some(Arc::new(|e| {
            // Filter out events where `e.environment == "development"` to prevent tracking errors on `localhost`.
            // See https://github.com/getsentry/sentry/issues/12341
            if e.environment.clone().is_some_and(|environment| environment.to_string() != "development") {
                Some(e)
            } else {
                None
            }
        })),
        ..Default::default()
    }));

    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

    // AWS S3 client
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let config = aws_config::defaults(BehaviorVersion::latest()).region(region_provider).load().await;
    let aws_client = web::Data::new(aws_sdk_s3::Client::new(&config));

    let prisma_client = web::Data::new(PrismaClient::_builder().build().await.unwrap());
    let redis_store = RedisSessionStore::new(env::var("REDIS_URL").unwrap()).await.unwrap();

    let player_stats = PlayerStats::create();
    let broadcaster = Broadcaster::create(player_stats.clone());
    let lumber_mill = LumberMill::create();
    let hourglass = Hourglass::create(prisma_client.clone(), broadcaster.clone(), player_stats.clone());

    env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at {}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(prisma_client.clone())
            .app_data(aws_client.clone())
            .app_data(broadcaster.clone())
            .app_data(lumber_mill.clone())
            .app_data(player_stats.clone())
            .app_data(hourglass.clone())
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(
                    redis_store.clone(),
                    Key::from(&[0; 64]),
                )
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(3)))
                .cookie_same_site(SameSite::None)
                .cookie_domain(env::var("DOMAIN").ok().and_then(|x| {
                    if x == "http://localhost:3000" {
                        None
                    } else {
                        Some(x)
                    }
                }))
                .build()
            )
            .wrap(Cors::permissive())
            .wrap(sentry_actix::Sentry::with_transaction())
            .configure(config_app)
    })
    .bind((host, port))?
    .run()
    .await
}
