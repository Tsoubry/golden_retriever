use serde::{Deserialize, Serialize};
//use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Deserialize, Serialize)]
//#[pg_mapper(table = "article")]
pub struct Article {
    pub platform: String,
    pub section: String,
    pub article_id: String,
    pub article_title: String,
    pub image_url: Option<String>,
    pub article_url: String,
    pub updated: i32
}
