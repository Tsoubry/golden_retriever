mod db;
mod models;

use tokio_postgres::{Error};

use log::{LevelFilter};
use simple_logger::SimpleLogger;

use db::get_client_pool;
use log::info;
use tokio_compat_02::FutureExt;


#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();


    let mut client = get_client_pool().get().compat().await.unwrap();

    let stmt = client.prepare("SELECT article_title FROM article").compat().await.unwrap();

    let result = client.query_one(&stmt, &[]).compat().await.unwrap();
    let title: String = result.get(0);

    info!("Article title is: {}", &title);

    Ok(())
}
