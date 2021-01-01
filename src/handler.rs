use netlify_lambda::{Context};
use serde::{Deserialize, Serialize};
use simple_error::bail;
use log::{error, info};
use uuid::Uuid;
use crate::error::Error;
use crate::db::get_client_pool;


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

    //let key = form_itemlist_key(e.target_service, e.section);

    // info!("getting current list based on key: {}", &key);
    // let current_list = get_item_list(&dynamo_client, key.clone()).await;
    // info!("current list acquired");
    //
    // match current_list {
    //     Ok(mut list) => {
    //         let mut new_items: Vec<String> = vec![Uuid::new_v4().to_string()];
    //         list.merge_lists(&mut new_items);
    //         add_itemlist(&dynamo_client, list).unwrap();
    //     },
    //     Err(_) => {
    //         log::info!("Key {} doesn't exist yet. Creating it", &key);
    //         add_itemlist(&dynamo_client, ItemList::new(key)).unwrap();
    //     }
    // }

    let mut client = get_client_pool().get().await.unwrap();

    let stmt = client.prepare("SELECT article_title FROM article").await.unwrap();

    let result = client.query_one(&stmt, &[]).await.unwrap();
    let title: String = result.get(0);

    info!("Article title is: {}", &title);

    Ok(CustomOutput {
        message: title,
    })
}