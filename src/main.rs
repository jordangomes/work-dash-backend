use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use actix_files::Files;
use sqlx::{sqlite::{SqlitePool}, migrate::MigrateDatabase};
use work_dash_backend::{
    AppState,
    users::{create_user_table}, 
    reminders::{create_reminder_table, create_reminder, get_active_reminders, get_all_reminders}
};


async fn not_found() -> impl Responder {
    HttpResponse::NotFound().content_type("text/html").body("<p>404 - These aren't the droids you are looking for</p>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = "sqlite://data.db";

    if !sqlx::Sqlite::database_exists(&db_url).await.expect("check if DB exists failed") {
        sqlx::Sqlite::create_database(&db_url).await.expect("create DB failed");
    }

    let pool = SqlitePool::connect(&db_url).await.expect("DB connection failed");

    create_user_table(&pool).await.expect("create user table failed");
    create_reminder_table(&pool).await.expect("create reminder table failed");

    create_reminder("Some Test reminder", "JG", &pool).await.expect("failed to create reminder");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { 
                db_pool: pool.clone()
            }))
            .default_service(web::route().to(not_found))
            .service(
                web::scope("/api")
                    .route("/reminders", web::get().to(get_all_reminders))
                    .route("/reminders/active", web::get().to(get_active_reminders))
            )
            .service(Files::new("/", "./www").prefer_utf8(true).index_file("index.html"))

    })
    .bind_openssl("127.0.0.1:3000", builder)?
    .run()
    .await
}