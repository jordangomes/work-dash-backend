use crate::reminders::ReminderError;
use crate::users::UserError;
use serde::Serialize;
use sqlx::{Pool, Sqlite, SqlitePool};
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use thiserror::Error;

pub mod users;
pub mod reminders;

pub struct AppState {
    pub db_pool: SqlitePool
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    ReminderError(#[from] ReminderError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    JSONError(#[from] serde_json::Error),
    #[error(transparent)]
    IOError(#[from] std::io::Error)
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String
}


impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ReminderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JSONError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::UserError(_) => todo!(),
        }
    }
    
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(AppErrorResponse { error: format!("{:?}", self) })
    }
}