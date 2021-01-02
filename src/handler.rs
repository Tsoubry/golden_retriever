use netlify_lambda::{Context};
use serde::{Deserialize, Serialize};
use simple_error::bail;
use log::{error, info};
use crate::error::Error;

use crate::db::{get_recent_articles, get_client_pool};

use tokio_compat_02::FutureExt;

use crate::services::tijd::{insert_all_articles, TIJD_URL, TIJD_PLATFORM, TIJD_SECTION};

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

    let recent_articles = get_recent_articles(&pool).compat().await;

    insert_all_articles(
        recent_articles,
        pool,
        TIJD_PLATFORM.to_string(),
        TIJD_SECTION.to_string()
    ).compat().await;

    Ok(CustomOutput {
        message: format!("process finished")
    })
}