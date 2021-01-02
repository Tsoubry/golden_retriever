use tokio_pg_mapper_derive::PostgresMapper;

#[derive(PostgresMapper)]
#[pg_mapper(table = "article")]
pub struct Article {
    pub id: i64,
    pub article_id: String,
    pub article_title: String,
    pub platform: String,
    pub section: String,
    pub image_url: Option<String>,
    pub article_url: Option<String>,
    pub updated: std::time::SystemTime,
}

#[derive(PostgresMapper)]
#[pg_mapper(table = "article")]
pub struct ArticleId {
    pub article_id: String,
}

pub struct ArticleInfo {
    pub article_id: String,
    pub article_title: String,
    pub image_url: Option<String>,
    pub article_url: Option<String>,
}