use actix_web::{Responder, HttpResponse, web};
use sqlx::{SqlitePool, Row};
use std::time::{SystemTime, SystemTimeError};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::AppState;

use crate::AppError;

#[derive(Error, Debug)]
pub enum TemperatureError {
    #[error(transparent)]
    TimeError(#[from] SystemTimeError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Temperature {
    id: u32,
    label: String,
    last_set_time: u32,
    temp: i32
}

pub async fn create_temperature_table(pool: &SqlitePool) -> Result<(), TemperatureError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS temperatures (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL, last_set_time INTEGER, temp INTEGER)")
        .execute(pool).await?;
    Ok(())
}

pub async fn create_temperature_probe(label: &str, pool: &SqlitePool) -> Result<(), TemperatureError> {
    sqlx::query("INSERT INTO temperatures (label, last_set_time, temp) values ($1, $2, $3)")
        .bind(label)
        .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
        .bind(255)
        .execute(pool).await?;

    Ok(())
}

pub async fn get_temperatures(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let query = sqlx::query("SELECT * FROM temperatures");
    let rows = query.fetch_all(&data.db_pool).await?;
    let temperatures: Vec<Temperature> = rows.iter().map(|row| {
        Temperature {
            id: row.get("id"),
            label: row.get("label"),
            last_set_time: row.get("last_set_time"),
            temp: row.get("temp")
        }
    }).collect();

    Ok(HttpResponse::Ok().json(temperatures))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateTemp {
    id: u32,
    temp: i32
}

pub async fn update_temperature(
    update: web::Json<UpdateTemp>,
    data: web::Data<AppState>
) -> Result<impl Responder, AppError> {
    sqlx::query("UPDATE temperatures SET temp = $1, last_set_time = $2 WHERE id = $3")
        .bind(update.temp)
        .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
        .bind(update.id)
        .execute(&data.db_pool).await?;

    Ok(HttpResponse::Ok().body("success"))
}