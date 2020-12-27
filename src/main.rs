mod dynamo;
mod retrieve;
mod services;
mod handler;

use lambda_runtime::lambda;
use log::{LevelFilter};
use simple_logger::SimpleLogger;

use handler::default_handler;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    lambda!(default_handler);

    Ok(())
}

