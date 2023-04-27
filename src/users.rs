use sqlx::{SqlitePool};
use thiserror::Error;
use bcrypt::{hash, verify, BcryptError};

const COST: u32 = 10;

#[derive(Error, Debug)]
pub enum UserError {
    #[error(transparent)]
    HashingError(#[from] BcryptError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

pub async fn create_user_table(pool: &SqlitePool) -> Result<(), UserError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE NOT NULL, initials TEXT UNIQUE NOT NULL, password TEXT NOT NULL )",)
        .execute(pool).await?;
    Ok(())
}

pub async fn create_user(username: &str, initials: &str, password: &str, pool: &SqlitePool) -> Result<(), UserError> {
    let secure_password = hash(password, COST)?;
    sqlx::query("INSERT INTO users (username, initials, password) values ($1, $2, $3)",)
        .bind(username)
        .bind(initials)
        .bind(secure_password)
        .execute(pool).await?;

    Ok(())
}