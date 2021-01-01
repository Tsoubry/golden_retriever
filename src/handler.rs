use netlify_lambda::{Context};
use serde::{Deserialize, Serialize};
use simple_error::bail;
use log::{error, info};
use uuid::Uuid;
use crate::error::Error;
use crate::db::{get_client_pool, add_article};
use std::time::SystemTime;
use crate::models::Article;

#[derive(Deserialize)]
pub struct CustomEvent {
    #[serde(rename = "targetService")]
    target_service: String,
    #[serde(rename = "section")]
    section: String,
}

#[derive(Serialize)]
pub struct CustomOutput {
    message: String,
}


pub async fn default_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, Error> {
    if e.target_service == "" || e.section == "" {
        error!(
            "Empty strings in one or more fields of the payload for request {}",
            c.request_id
        );
        bail!("Empty fields");
    }

    info!("getting current list for target: {}, section: {}", &e.target_service, &e.section);

    let pool = get_client_pool();

    let fake_article = Article {
        id: 1,
        article_id: uuid::Uuid::new_v4().to_string(),
        article_title: "another article".to_string(),
        platform: e.target_service,
        section: e.section,
        image_url: Some("".to_string()),
        article_url: Some("google.com".to_string()),
        updated: SystemTime::now(),
    };

    let result = add_article(&pool, fake_article).await.unwrap();

    Ok(CustomOutput {
        message: format!("result: {}", result),
    })
}