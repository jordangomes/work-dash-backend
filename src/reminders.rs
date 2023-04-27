use sqlx::{SqlitePool, Row};
use std::time::{SystemTime, SystemTimeError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReminderError {
    #[error(transparent)]
    TimeError(#[from] SystemTimeError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

#[derive(Serialize, Deserialize, Debug)]
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

pub async fn create_reminder(reminder: &str, user_initials: &str, pool: &SqlitePool) -> Result<(), ReminderError> {
    sqlx::query("INSERT INTO reminders (created_time, active, reminder, user_initials) values ($1, $2, $3, $4)")
        .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
        .bind(true)
        .bind(reminder)
        .bind(user_initials)
        .execute(pool).await?;

    Ok(())
}

pub async fn get_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, ReminderError> {
    let query = sqlx::query("SELECT * FROM reminders ORDER BY created_time ASC");
    let rows = query.fetch_all(pool).await?;
    let reminders = rows.iter().map(|row| {
        Reminder {
            id: row.get("id"),
            created_time: row.get("created_time"),
            active: row.get("active"),
            reminder: row.get("reminder"),
            user_initials: row.get("user_initials")
        }
    }).collect();

    Ok(reminders)
}

pub async fn get_active_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, ReminderError> {
    let query = sqlx::query("SELECT * FROM reminders WHERE active = 1 ORDER BY created_time ASC");
    let rows = query.fetch_all(pool).await?;
    let reminders = rows.iter().map(|row| {
        Reminder {
            id: row.get("id"),
            created_time: row.get("created_time"),
            active: row.get("active"),
            reminder: row.get("reminder"),
            user_initials: row.get("user_initials")
        }
    }).collect();

    Ok(reminders)
}

pub async fn disable_reminder(id: u32, user_initials: &str, pool: &SqlitePool) -> Result<(), ReminderError> {
    sqlx::query("UPDATE reminders SET active = $1 WHERE id = $2")
        .bind(false)
        .bind(id)
        .execute(pool).await?;

    Ok(())
}