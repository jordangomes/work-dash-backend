use actix_web::Result;
use actix_web::{Responder, HttpResponse, web};
use sqlx::{SqlitePool};
use std::time::{SystemTime, Duration, SystemTimeError};
use std::num::TryFromIntError;
use std::net::{IpAddr, Ipv4Addr, AddrParseError};
use dns_lookup::{lookup_host};
use surge_ping::{Client, Config, PingIdentifier, PingSequence, ICMP};
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::AppState;

use crate::AppError;

#[derive(Error, Debug)]
pub enum PingError {
    #[error(transparent)]
    TimeError(#[from] SystemTimeError),
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error)
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Ping {
    id: u32,
    label: String,
    address: String,
    last_set_time: u32,
    ping: i32,
    error: String
}

pub async fn create_ping_table(pool: &SqlitePool) -> Result<(), PingError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS ping (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL, address TEXT NOT NULL, last_set_time INTEGER, ping INTEGER, error TEXT)")
        .execute(pool).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewPing {
    label: String,
    address: String,
}
pub async fn create_ping(
    ping: web::Json<NewPing>,
    data: web::Data<AppState>,
)  -> Result<impl Responder, AppError> {
    let query = sqlx::query_as::<_, Ping>("INSERT INTO ping (label, address, last_set_time, ping) values ($1, $2, $3, $4) RETURNING *")
        .bind(ping.label.clone())
        .bind(ping.address.clone())
        .bind(0)
        .bind(-1);

    let row: Ping = query.fetch_one(&data.db_pool).await?;

    Ok(HttpResponse::Ok().json(row))
}

pub async fn get_ping(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let query = sqlx::query_as::<_, Ping>("SELECT * FROM ping");
    let rows: Vec<Ping> = query.fetch_all(&data.db_pool).await?;
    Ok(HttpResponse::Ok().json(rows))
}

pub async fn ping_hosts(pool: &SqlitePool) -> Result<(), PingError> {
    let query = sqlx::query_as::<_, Ping>("SELECT * FROM ping");
    let hosts: Vec<Ping> = query.fetch_all(pool).await?;

    for host in hosts {
        let result = match resolve_address(&host.address).await {
            Ok(ip) => { 
                let mut config_builder = Config::builder();
                if ip.is_ipv6() {
                    config_builder = config_builder.kind(ICMP::V6);
                }
                let config = config_builder.build();
                let payload = vec![0; 64];
                let client = Client::new(&config).unwrap();
                let mut pinger = client.pinger(ip, PingIdentifier(111)).await;
                pinger.timeout(Duration::from_secs(2));
                match pinger.ping(PingSequence(0), &payload).await {
                    Ok((_, rtt)) => {
                        update_ping(host.id, rtt.as_millis(), "", pool).await
                    }
                    Err(e) => { update_ping(host.id, 0, &format!("{:?}", e), pool).await },
                }
            },
            Err(e) => {
                update_ping(host.id, 0, &format!("{:?}", e), pool).await
            }
        };

        match result {
            Ok(_) => {},
            Err(e) => println!("{:?}", e)
        };
    }
    Ok(())
}

pub async fn update_ping(
    id: u32,
    ping: u128,
    error: &str,
    pool: &SqlitePool
) -> Result<(), PingError> {
    let reasonable_ping: u32  = ping.try_into()?;
    sqlx::query("UPDATE ping SET ping = $1, error = $2, last_set_time = $3 WHERE id = $4")
        .bind(reasonable_ping)
        .bind(error)
        .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
        .bind(id)
        .execute(pool).await?;

    Ok(())
}

pub async fn resolve_address(address: &str) -> Result<IpAddr, PingError> {
    let parse_ip: Result<Ipv4Addr, AddrParseError> = address.parse();
    match parse_ip {
        Ok(ip) => { return Ok(std::net::IpAddr::V4(ip)) },
        Err(_) => {
            let ips = lookup_host(&address)?;
            return Ok(ips[0])
        }
    }
}

