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

use handler::default_handler;


#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = handler_fn(default_handler);
    run(func).await.unwrap();

    Ok(())
}

