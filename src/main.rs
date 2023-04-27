// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{sqlite::{SqlitePool}, migrate::MigrateDatabase};
use thiserror::Error;
use work_dash_backend::users::{UserError, create_user, create_user_table};
use work_dash_backend::reminders::{ReminderError, get_active_reminders, disable_reminder, create_reminder_table};


#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ReminderError(#[from] ReminderError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    JSONError(#[from] serde_json::Error)
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    let db_url = "sqlite://data.db";

    if !sqlx::Sqlite::database_exists(&db_url).await? {
        sqlx::Sqlite::create_database(&db_url).await?;
    }

    let pool = SqlitePool::connect(&db_url).await?;

    create_user_table(&pool).await?;
    create_reminder_table(&pool).await?;

    disable_reminder(1, "JG", &pool).await?;

    let reminders = get_active_reminders(&pool).await?;
    println!("{}", serde_json::to_string(&reminders)?);

    Ok(())
}