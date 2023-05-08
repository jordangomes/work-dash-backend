use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use actix_web_httpauth::middleware::HttpAuthentication;
use tokio_schedule::{every, Job};
use chrono::{Local, Utc};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use actix_files::Files;
use sqlx::{sqlite::{SqlitePool}, migrate::MigrateDatabase};
use work_dash_backend::{
    AppState,
    users::{create_user_table, login, bearer_auth_validator, get_api_key}, 
    reminders::{create_reminder_table, get_active_reminders, get_all_reminders, create_reminder, disable_reminder},
    temperatures::{create_temperature_table, update_temperature, get_temperatures}, 
    rss::{create_rss_feed_table, get_feeds, download_rss_feeds, create_rss_feed_item_table, get_feed_items, create_rss_feed, dismiss_feed_item}, ping::{create_ping_table, create_ping, get_ping, ping_hosts}
};


async fn not_found() -> impl Responder {
    HttpResponse::NotFound().content_type("text/html").body("<p>404 - These aren't the droids you are looking for</p>")
}


async fn start_rss_scheduler(pool: &SqlitePool) {
    match download_rss_feeds(&pool).await {
        Ok(_) => println!("First data refresh succeeded"),
        Err(e) => println!("First data refresh failed - {}", e)
    }
    let every_hour = every(30)
        .minutes()
        .in_timezone(&Utc)
        .perform(|| async { 
            println!("schedule_task data refresh - {:?}", Local::now()); 
            match download_rss_feeds(&pool).await {
                Ok(_) => println!("data refresh succeeded"),
                Err(e) => println!("data refresh failed - {}", e)
            }
        });
    every_hour.await;
}

async fn start_ping_scheduler(pool: &SqlitePool) {
    match ping_hosts(&pool).await {
        Ok(_) => println!("First ping task succeeded"),
        Err(e) => println!("First ping task failed - {}", e)
    }
    let every_hour = every(30)
        .seconds()
        .in_timezone(&Utc)
        .perform(|| async { 
            println!("schedule_task ping - {:?}", Local::now()); 
            match ping_hosts(&pool).await {
                Ok(_) => println!("ping task succeeded"),
                Err(e) => println!("ping task failed - {}", e)
            }
        });
    every_hour.await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let jwt_key_string: String = std::env::var("JWT_KEY").expect("JWT_KEY environment variable is not set");
    let jwt_key: Hmac<Sha256> = Hmac::<Sha256>::new_from_slice(jwt_key_string.as_bytes()).expect("HMAC can take key of any size");

    let db_url = "sqlite://data.db";

    if !sqlx::Sqlite::database_exists(&db_url).await.expect("check if DB exists failed") {
        sqlx::Sqlite::create_database(&db_url).await.expect("create DB failed");
    }

    let pool = SqlitePool::connect(&db_url).await.expect("DB connection failed");
    let rss_pool = SqlitePool::connect(&db_url).await.expect("DB connection failed");
    let ping_pool = SqlitePool::connect(&db_url).await.expect("DB connection failed");



    create_user_table(&pool).await.expect("create user table failed");
    create_reminder_table(&pool).await.expect("create reminder table failed");
    create_temperature_table(&pool).await.expect("create temperature table failed");
    create_rss_feed_table(&pool).await.expect("create rss feed table failed");
    create_rss_feed_item_table(&pool).await.expect("create rss feed item table failed");
    create_ping_table(&pool).await.expect("create ping table failed");

    ping_hosts(&pool).await.expect("error pinging hosts");

    
    actix_rt::spawn(async move {
        start_rss_scheduler(&rss_pool).await;
    });
    actix_rt::spawn(async move {
        start_ping_scheduler(&ping_pool).await;
    });
    


    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let auth = HttpAuthentication::bearer(bearer_auth_validator);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { 
                db_pool: pool.clone(),
                jwt_key: jwt_key.clone()
            }))
            .default_service(web::route().to(not_found))
            .route("/api/login", web::post().to(login))
            .service(
                web::scope("/api")
                    .wrap(Logger::default())
                    .wrap(auth.clone())
                    .route("/apikey", web::get().to(get_api_key))
                    .route("/reminders", web::get().to(get_all_reminders))
                    .route("/reminders", web::post().to(create_reminder))
                    .route("/reminders", web::delete().to(disable_reminder))
                    .route("/reminders/active", web::get().to(get_active_reminders))
                    .route("/temperatures", web::get().to(get_temperatures))
                    .route("/temperatures", web::post().to(update_temperature))
                    .route("/rss/feeds", web::get().to(get_feeds))
                    .route("/rss/feed", web::get().to(get_feed_items))
                    .route("/rss/feed", web::post().to(create_rss_feed))
                    .route("/rss/feed/dismiss", web::post().to(dismiss_feed_item))
                    .route("/ping", web::get().to(get_ping))
                    .route("/ping", web::post().to(create_ping))



            )
            .service(Files::new("/", "./www").prefer_utf8(true).index_file("index.html"))

    })
    .bind_openssl("127.0.0.1:3000", builder)?
    .run()
    .await
}