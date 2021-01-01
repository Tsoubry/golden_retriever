mod db;
mod retrieve;
mod services;
mod handler;
mod error;
mod models;

use tokio_postgres::{Error};

use netlify_lambda::{handler_fn, run};
use log::{LevelFilter};
use simple_logger::SimpleLogger;
use log::info;

use db::get_client_pool;

use handler::default_handler;



#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let mut client = get_client_pool().get().await.unwrap();

    let stmt = client.prepare("SELECT article_title FROM article").await.unwrap();

    let result = client.query_one(&stmt, &[]).await.unwrap();
    let title: String = result.get(0);

    info!("Article title is: {}", title);

    //let func = handler_fn(default_handler);
    //run(func).await?;

    Ok(())
}

