use log::info;

use tokio_postgres::{NoTls, Row};
use dotenv::dotenv;
use deadpool_postgres::Pool;

use crate::models::Article;
use tokio_postgres::error::Error;

mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}


pub fn get_client_pool() -> Pool {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();

    config.pg.create_pool(NoTls).unwrap()

}

pub async fn add_article(pool: &Pool, article: Article) -> Result<u64, Error>{
    let client = pool.get().await.unwrap();
    let stmt = client.prepare(
        "INSERT INTO article (article_id, article_title, platform, section, image_url, article_url, updated) \
        VALUES ($1, $2, $3, $4, $5, $6, $7)").await.unwrap();

    client.execute(&stmt, &[
        &article.article_id,
        &article.article_title,
        &article.platform,
        &article.section,
        &article.image_url.unwrap(),
        &article.article_url.unwrap(),
        &article.updated,
    ]).await

}


