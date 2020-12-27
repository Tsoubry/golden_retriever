use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, PutItemOutput};
use serde_dynamodb;
use serde_dynamodb::Error;
use std::collections::HashMap;

#[macro_use]
use serde::{Deserialize, Serialize};

use rusoto_core::Region;
use uuid::Uuid;
use futures::executor;

const TABLE_NAME: &str = "corgi";
const MAX_ITEMS_LIST: usize = 300;

#[derive(Serialize, Deserialize)]
pub struct ItemList {
    item_id: String,
    items: Vec<Uuid>,
}

impl ItemList {
    pub fn new(key: String) -> Self {
        ItemList {
            item_id: key,
            items: Vec::new(),
        }
    }

    pub fn merge_lists(&mut self, new_items: &mut Vec<Uuid>) {
        self.items.append(new_items);
        self.items.truncate(MAX_ITEMS_LIST);
    }
}

pub fn form_itemlist_key(service: String, section: String) -> String {
    format!("{}_{}", service, section)
}

pub fn get_client() -> DynamoDbClient {
    DynamoDbClient::new(Region::EuCentral1)

}

pub async fn get_item_list(client: &DynamoDbClient, key: String) -> Result<ItemList, Error> {

    let mut query = HashMap::new();
    query.insert(
        String::from(":item_id"),
        AttributeValue {
            s: Some(key),
            ..Default::default()
        },
    );

    let item_fut = client.get_item(GetItemInput {
        key: query,
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    });

    let hashmap = executor::block_on(item_fut)
        .unwrap()
        .item
        .unwrap();

    let item_list: ItemList = serde_dynamodb::from_hashmap(hashmap).unwrap();

    Ok(item_list)
}

pub fn add_itemlist(client: &DynamoDbClient, item_list: ItemList) -> Result<(), Error> {
    let fut_result = client.put_item(PutItemInput {
        item: serde_dynamodb::to_hashmap(&item_list)?,
        table_name: TABLE_NAME.to_string(),
        ..Default::default()
    });

    executor::block_on(fut_result).unwrap();

    Ok(())

}
