use hmac::Hmac;
use sha2::Sha256;
use sqlx::{SqlitePool};
use std::{
    collections::{ BTreeMap },
    time::{SystemTime, SystemTimeError }, 
};
use thiserror::Error;
use bcrypt::{hash, verify, BcryptError};
use jwt::{SignWithKey, VerifyWithKey, Header, Token};
use actix_web::{HttpResponse, Responder, dev::ServiceRequest, web};
use actix_web_httpauth::extractors::{ bearer::{BearerAuth, Config}, AuthenticationError };
use serde::{Serialize,Deserialize};
use crate::AppState;

const COST: u32 = 10;

#[derive(Error, Debug)]
pub enum UserError {
    #[error(transparent)]
    HashingError(#[from] BcryptError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    initials: String,
    password: String,
    is_admin: bool
}

pub async fn create_user_table(pool: &SqlitePool) -> Result<(), UserError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT UNIQUE NOT NULL, initials TEXT UNIQUE NOT NULL, password TEXT NOT NULL, is_admin INTEGER)",)
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

fn get_time_since_epoch() -> Result<u64, SystemTimeError> {
    Ok(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs())
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    token: String
}

pub async fn login(
    data: web::Data<AppState>,
    login_attempt: web::Json<LoginRequest>,
) -> impl Responder {
    let query = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username=$1").bind(&login_attempt.username);
    let user_record: User = match query.fetch_optional(&data.db_pool).await {
        Ok(rows) => { 
            match rows {
                Some(row) => row,
                None => return HttpResponse::Forbidden().body("403 - Incorrect username or password")
            } 
        }
        Err(_) => return HttpResponse::Forbidden().body("403 - Login Failed")
    };

    let password_matches: bool = match verify(&login_attempt.password, &user_record.password) {
        Ok(password_matches) => { password_matches },
        Err(_) => return HttpResponse::Forbidden().body("403 - Login Failed")
    };

    match password_matches {
        true => {
            match get_time_since_epoch() {
                Ok(time) => {
                    let is_admin: String = match user_record.is_admin {
                        true => "1".to_string(),
                        false => "0".to_string()
                    };
                    let mut claims = BTreeMap::new();
                    let iat = time.to_string();
                    let exp = (time + 3600).to_string();
                    claims.insert("iss", "workdashboard.com");
                    claims.insert("aud", "apps");
                    claims.insert("sub", &user_record.username);
                    claims.insert("initials", &user_record.initials);
                    claims.insert("is_admin", &is_admin);
                    claims.insert("iat", &iat);
                    claims.insert("nbf", &iat);
                    claims.insert("exp", &exp);
        
                    let token = match claims.sign_with_key(&data.jwt_key) {
                        Ok(token) => token,
                        Err(_) => return HttpResponse::InternalServerError().body("500 - Unexpected error generating token")
                    };

                    HttpResponse::Ok().json(LoginResponse { token })
                },
                Err(_e) => {
                    HttpResponse::InternalServerError().body("500 - Unexpected error generating token")
                }
            }
        }
        false => {
            HttpResponse::Forbidden().body("403 - Incorrect username or password")
        }
    }
}

pub async fn get_api_key(
    data: web::Data<AppState>,
    auth: BearerAuth
) -> impl Responder {
    let token: Option<UserToken> = parse_token(auth.token(), data.jwt_key.clone());
    let valid_token = match token {
        Some(token) => { token },
        None => return HttpResponse::Forbidden().body("Unable to validate User Identity")
    };

    match get_time_since_epoch() {
        Ok(time) => {
            if valid_token.is_admin == "1" {
                let mut claims = BTreeMap::new();
                let iat = time.to_string();
                claims.insert("iss", "workdashboard.com");
                claims.insert("aud", "apps");
                claims.insert("sub", &valid_token.sub);
                claims.insert("initials", &valid_token.initials);
                claims.insert("is_admin", "0");
                claims.insert("iat", &iat);
                claims.insert("nbf", &iat);

                let token = match claims.sign_with_key(&data.jwt_key) {
                    Ok(token) => token,
                    Err(_) => return HttpResponse::InternalServerError().body("500 - Unexpected error generating token")
                };

                HttpResponse::Ok().json(LoginResponse { token })
            } else {
                return HttpResponse::Forbidden().body("Only admin users can generate API keys");
            }
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("500 - Unexpected error generating token")
        }
    }
}

pub async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.as_ref().clone())
        .unwrap_or_else(Default::default);

    let app_state: &AppState = req.app_data::<web::Data<AppState>>().expect("AppState missing in request handler.");

    let token_str = credentials.token();
    let token: Result<Token<Header, BTreeMap<String, String>, _>,_> = VerifyWithKey::verify_with_key(token_str, &app_state.jwt_key);

    match token {
        Ok(valid_token) => {
            // get seconds since the unix epoch default to the year 5138
            let time_since_epoch: u64 = match get_time_since_epoch() {
                Ok(time) => time,
                Err(_) => 99999999999
            };

            // get expiry of jwt token default to unix epoch on error 
            let exp: u64 = match valid_token.claims().contains_key("exp") {
                true => {
                    match valid_token.claims()["exp"].parse() {
                        Ok(value) => value,
                        Err(_) => 0
                    }
                }
                false => u32::MAX as u64
            };

            if time_since_epoch > exp  {
                return Err((AuthenticationError::new(config).into(), req));
            }

            let query = sqlx::query("SELECT * FROM users WHERE username=$1").bind(valid_token.claims()["sub"].clone());
            let rows = match query.fetch_optional(&app_state.db_pool).await {
                Ok(rows) => { rows }
                Err(_) => { return Err((AuthenticationError::new(config).into(), req)); }
            };

            match rows {
                Some(_) => {},
                None => { return Err((AuthenticationError::new(config).into(), req)); }
            }

            Ok(req)
        },
        Err(_e) => Err((AuthenticationError::new(config).into(), req))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    pub iss: String,
    pub aud: String,
    pub sub: String,
    pub initials: String,
    pub iat: String,
    pub nbf: String,
    pub exp: String,
    pub is_admin: String
}

pub fn parse_token(token: &str, key: Hmac<Sha256>) -> Option<UserToken> {
    let token: Result<Token<Header, BTreeMap<String, String>, _>,_> = VerifyWithKey::verify_with_key(token, &key);
    match token {
        Ok(token) => {
            Some(UserToken {
                iss: token.claims()["iss"].clone(),
                aud: token.claims()["aud"].clone(),
                sub: token.claims()["sub"].clone(),
                initials: token.claims()["initials"].clone(),
                iat: token.claims()["iat"].clone(),
                nbf: token.claims()["nbf"].clone(),
                exp: token.claims()["exp"].clone(),
                is_admin: token.claims()["is_admin"].clone()
            })
        }
        Err(_) => None
    }
}