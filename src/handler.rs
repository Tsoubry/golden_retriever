use lambda_runtime::{error::HandlerError, Context};
use serde::{Deserialize, Serialize};
use simple_error::bail;
use log::{error, info};
use uuid::Uuid;

use crate::dynamo::{get_client, get_item_list, add_itemlist, form_itemlist_key, ItemList};


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


pub fn default_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
    if e.target_service == "" || e.section == "" {
        error!(
            "Empty strings in one or more fields of the payload for request {}",
            c.aws_request_id
        );
        bail!("Empty fields");
    }

    info!("acquiring dynamo client");
    let dynamo_client = get_client();
    info!("dynamo client acquired. ");
    let key = form_itemlist_key(e.target_service, e.section);

    info!("getting current list based on key: {}", &key);
    let current_list = get_item_list(&dynamo_client, key.clone());
    info!("current list acquired");

    match current_list {
        Ok(mut list) => {
            let mut new_items: Vec<Uuid> = vec![Uuid::new_v4()];
            list.merge_lists(&mut new_items);
            add_itemlist(&dynamo_client, list).unwrap();
        },
        Err(_) => {
            log::info!("Key {} doesn't exist yet. Creating it", &key);
            add_itemlist(&dynamo_client, ItemList::new(key)).unwrap();
        }
    }


    Ok(CustomOutput {
        message: format!("Succeeded"),
    })
}