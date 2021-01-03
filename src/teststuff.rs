mod retrieve;
mod db;
mod models;
mod services;
use tokio_postgres::{Error};


use log::{LevelFilter};
use simple_logger::SimpleLogger;

use db::{get_recent_articles, get_client_pool};

use tokio_compat_02::FutureExt;

use services::tijd::{insert_all_articles, TIJD_PLATFORM, TIJD_SECTION};

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let pool = get_client_pool();

    let recent_articles = get_recent_articles(&pool, TIJD_PLATFORM, TIJD_SECTION).compat().await;

    insert_all_articles(
        recent_articles,
        pool,
        TIJD_PLATFORM.to_string(),
        TIJD_SECTION.to_string(),
    )
    .compat()
    .await;

    Ok(())
}
