use actix_web::{Responder, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sqlx::{SqlitePool, Row};
use std::time::{SystemTime, SystemTimeError};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::AppState;
use crate::AppError;
use crate::users::UserToken;
use crate::users::parse_token;

#[derive(Error, Debug)]
pub enum ReminderError {
    #[error(transparent)]
    TimeError(#[from] SystemTimeError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Reminder {
    id: u32,
    created_time: u32,
    active: bool,
    reminder: String,
    user_initials: String
}

pub async fn create_reminder_table(pool: &SqlitePool) -> Result<(), ReminderError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS reminders (id INTEGER PRIMARY KEY AUTOINCREMENT, created_time INTEGER, active INTEGER, reminder TEXT NOT NULL, user_initials TEXT NOT NULL)")
        .execute(pool).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewReminder {
    reminder: String
}

pub async fn create_reminder(
    reminder: web::Json<NewReminder>,
    data: web::Data<AppState>, 
    auth: BearerAuth
)  -> Result<impl Responder, AppError> {
    let token: Option<UserToken> = parse_token(auth.token(), data.jwt_key.clone());
    let valid_token = match token {
        Some(token) => { token },
        None => return Ok(HttpResponse::Forbidden().body("Unable to validate User Identity"))
    };

    let query = sqlx::query_as::<_, Reminder>("INSERT INTO reminders (created_time, active, reminder, user_initials) values ($1, $2, $3, $4) RETURNING *")
        .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
        .bind(true)
        .bind(reminder.reminder.clone())
        .bind(valid_token.initials);
    
    let row: Reminder = query.fetch_one(&data.db_pool).await?;

    Ok(HttpResponse::Ok().json(row))
}

pub async fn get_active_reminders(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let query = sqlx::query("SELECT * FROM reminders WHERE active = 1  ORDER BY created_time ASC");
    let rows = query.fetch_all(&data.db_pool).await?;
    let reminders: Vec<Reminder> = rows.iter().map(|row| {
        Reminder {
            id: row.get("id"),
            created_time: row.get("created_time"),
            active: row.get("active"),
            reminder: row.get("reminder"),
            user_initials: row.get("user_initials")
        }
    }).collect();

    Ok(HttpResponse::Ok().json(reminders))
}

pub async fn get_all_reminders(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let query = sqlx::query("SELECT * FROM reminders  ORDER BY created_time ASC");
    let rows = query.fetch_all(&data.db_pool).await?;
    let reminders: Vec<Reminder> = rows.iter().map(|row| {
        Reminder {
            id: row.get("id"),
            created_time: row.get("created_time"),
            active: row.get("active"),
            reminder: row.get("reminder"),
            user_initials: row.get("user_initials")
        }
    }).collect();

    Ok(HttpResponse::Ok().json(reminders))
}


// pub async fn disable_reminder(id: u32, _user_initials: &str, pool: &SqlitePool) -> Result<(), ReminderError> {


//     Ok(())
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct DisableReminder {
    id: u32
}

pub async fn disable_reminder(
    disable: web::Json<DisableReminder>,
    data: web::Data<AppState>, 
    auth: BearerAuth
)  -> Result<impl Responder, AppError> {
    let token: Option<UserToken> = parse_token(auth.token(), data.jwt_key.clone());
    let valid_token = match token {
        Some(token) => { token },
        None => return Ok(HttpResponse::Forbidden().body("Unable to validate User Identity"))
    };

    let query = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders WHERE id = $1").bind(disable.id);
    let row: Reminder = query.fetch_one(&data.db_pool).await?;

    if valid_token.initials == row.user_initials {
        sqlx::query("UPDATE reminders SET active = $1 WHERE id = $2")
            .bind(false)
            .bind(disable.id)
            .execute(&data.db_pool).await?;
    } else {
        return Ok(HttpResponse::Forbidden().body(format!("{} is unable to delete reminders for {}", valid_token.initials, row.user_initials)));
    }


    Ok(HttpResponse::Ok().body("success"))
}
