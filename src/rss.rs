use actix_web::http::Error;
use actix_web::{Responder, HttpResponse, web};
use rss::{Channel, Item};
use sqlx::{SqlitePool, Row};
use std::time::{SystemTimeError};
use chrono::{DateTime};
use chrono::format::ParseError;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use crate::AppState;

use crate::AppError;

#[derive(Error, Debug)]
pub enum RSSError {
    #[error(transparent)]
    TimeError(#[from] SystemTimeError),
    #[error(transparent)]
    ChronoTimeError(#[from] ParseError),
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    #[error(transparent)]
    RSSError(#[from] rss::Error)
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct RssFeed {
    id: u32, 
    label: String, 
    url: String, 
    important: bool
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct RssFeedItem {
    id: u32,
    important: bool,
    dismissed: bool,
    source_label: String,
    pub_date: u32,
    guid: String,
    title: String,
    link: String,
    description: String,
    categories: String
}

pub async fn create_rss_feed_table(pool: &SqlitePool) -> Result<(), RSSError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS rss_feeds (id INTEGER PRIMARY KEY AUTOINCREMENT, label TEXT NOT NULL, url TEXT NOT NULL, important INTEGER)")
        .execute(pool).await?;
    Ok(())
}
pub async fn create_rss_feed_item_table(pool: &SqlitePool) -> Result<(), RSSError> {
    sqlx::query("CREATE TABLE IF NOT EXISTS rss_feed_items (id INTEGER PRIMARY KEY AUTOINCREMENT, important INTEGER, dismissed INTEGER, source_label TEXT NOT NULL, pub_date INTEGER, guid TEXT NOT NULL, title TEXT NOT NULL, link TEXT NOT NULL, description TEXT, categories TEXT)")
        .execute(pool).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NewRSSFeed {
    label: String,
    url: String,
    important: bool
}

pub async fn create_rss_feed(
    feed: web::Json<NewRSSFeed>,
    data: web::Data<AppState>
)   -> Result<impl Responder, AppError> {
    let query = sqlx::query_as::<_, RssFeed>("INSERT INTO rss_feeds (label, url, important) values ($1, $2, $3) RETURNING *")
        .bind(feed.label.clone())
        .bind(feed.url.clone())
        .bind(feed.important.clone());

    let row: RssFeed = query.fetch_one(&data.db_pool).await?;

    Ok(HttpResponse::Ok().json(row))
}

pub async fn get_feeds(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let query = sqlx::query("SELECT * FROM rss_feeds");
    let rows = query.fetch_all(&data.db_pool).await?;
    let feeds: Vec<RssFeed> = rows.iter().map(|row| {
        RssFeed {
            id: row.get("id"),
            label: row.get("label"),
            url: row.get("url"),
            important: row.get("important")
        }
    }).collect();

    Ok(HttpResponse::Ok().json(feeds))
}

pub async fn get_feed_items(data: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let important_query = sqlx::query_as::<_, RssFeedItem>("SELECT * FROM rss_feed_items WHERE important=1 AND dismissed=0 ORDER BY pub_date DESC LIMIT 20");
    let mut important_rows: Vec<RssFeedItem> = important_query.fetch_all(&data.db_pool).await?;
    let last20_query = sqlx::query_as::<_, RssFeedItem>("SELECT * FROM rss_feed_items ORDER BY pub_date DESC LIMIT 20");
    let mut last20_rows: Vec<RssFeedItem> = last20_query.fetch_all(&data.db_pool).await?;
    
    let mut result: Vec<RssFeedItem> = vec![];
    result.append(&mut important_rows);
    result.append(&mut last20_rows);

    Ok(HttpResponse::Ok().json(result))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DismissFeedItem {
    id: u32
}

pub async fn dismiss_feed_item(
    disable: web::Json<DismissFeedItem>,
    data: web::Data<AppState>
)  -> Result<impl Responder, AppError> {

    let query = sqlx::query_as::<_, RssFeedItem>("SELECT * FROM rss_feed_items WHERE id = $1").bind(disable.id);
    let row: RssFeedItem = query.fetch_one(&data.db_pool).await?;

    sqlx::query("UPDATE rss_feed_items SET dismissed = $1 WHERE id = $2")
        .bind(true)
        .bind(disable.id)
        .execute(&data.db_pool).await?;


    Ok(HttpResponse::Ok().body("success"))
}


pub async fn download_rss_feeds(pool: &SqlitePool) -> Result<(), RSSError> {
    let query = sqlx::query("SELECT * FROM rss_feeds");
    let rows = query.fetch_all(pool).await?;
    let feeds: Vec<RssFeed> = rows.iter().map(|row| {
        RssFeed {
            id: row.get("id"),
            label: row.get("label"),
            url: row.get("url"),
            important: row.get("important")
        }
    }).collect();

    for feed in feeds {
        let content = reqwest::get(&feed.url)
            .await?
            .bytes()
            .await?;

        match Channel::read_from(&content[..]) {
            Ok(channel) => {
                for article in channel.items {
                    match validate_and_insert_feed_item(&feed, &article, pool).await {
                        Ok(_) => {}
                        Err(e) => { println!("Error adding feed item - {:?}", e) }
                    }
                }
            },
            Err(e) => println!("unable to read data from {} - \n {:?}", &feed.url, e)
        };
        

    }
    Ok(())
}

pub async fn validate_and_insert_feed_item(feed: &RssFeed, feed_item: &Item, pool: &SqlitePool) -> Result<(), RSSError> {
    match (feed_item.pub_date.clone(), feed_item.guid.clone(), feed_item.title.clone(), feed_item.link.clone()) {   
        (Some(pub_date), Some(guid), Some(title), Some(link)) => {
            let description: String = feed_item.description.clone().unwrap_or("".to_string());
            let categories: String = feed_item.categories.iter().map(|x| x.name.to_string() + ",").collect::<String>();
            let parsed_pub_date = DateTime::parse_from_rfc2822(&pub_date)?;

            let query = sqlx::query("SELECT * FROM rss_feed_items WHERE guid=$1").bind(&guid.value);
            let rows = query.fetch_optional(pool).await?;
            match rows {
                Some(_) => {},
                None => {
                    println!("Adding {} from {}", &title, feed.label);
                    sqlx::query("INSERT INTO rss_feed_items (important, dismissed, source_label, pub_date, guid, title, link, description, categories) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
                        .bind(feed.important)
                        .bind(false)
                        .bind(&feed.label)
                        .bind(parsed_pub_date.timestamp())
                        .bind(guid.value)
                        .bind(title)
                        .bind(link)
                        .bind(description)
                        .bind(categories)
                        .execute(pool).await?;

                }
            }
        }
        (Some(pub_date), None, Some(title), Some(link)) => {
            let description: String = feed_item.description.clone().unwrap_or("".to_string());
            let categories: String = feed_item.categories.iter().map(|x| x.name.to_string() + ",").collect::<String>();
            let parsed_pub_date = DateTime::parse_from_rfc2822(&pub_date)?;

            let query = sqlx::query("SELECT * FROM rss_feed_items WHERE guid=$1").bind(&link);
            let rows = query.fetch_optional(pool).await?;
            match rows {
                Some(_) => {},
                None => {
                    println!("Adding {} from {}", &title, feed.label);
                    sqlx::query("INSERT INTO rss_feed_items (important, dismissed, source_label, pub_date, guid, title, link, description, categories) values ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
                        .bind(feed.important)
                        .bind(false)
                        .bind(&feed.label)
                        .bind(parsed_pub_date.timestamp())
                        .bind(link.clone())
                        .bind(title)
                        .bind(link.clone())
                        .bind(description)
                        .bind(categories)
                        .execute(pool).await?;

                }
            }
        }
        _ => { println!("Feed Item missing require parameter {:?}", feed_item) }
    }
    Ok(())
}


// #[derive(Serialize, Deserialize, Clone)]
// pub struct UpdateTemp {
//     id: u32,
//     temp: i32
// }

// pub async fn update_temperature(
//     update: web::Json<UpdateTemp>,
//     data: web::Data<AppState>
// ) -> Result<impl Responder, AppError> {
//     sqlx::query("UPDATE temperatures SET temp = $1, last_set_time = $2 WHERE id = $3")
//         .bind(update.temp)
//         .bind(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs() as u32)
//         .bind(update.id)
//         .execute(&data.db_pool).await?;

//     Ok(HttpResponse::Ok().body("success"))
// }