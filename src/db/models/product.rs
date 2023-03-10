use super::{common::DBModel, store::Store};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub description: String,
    pub name: String,
    pub store: StoreField,
    pub sizes: Vec<String>,
    pub variants: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StoreField {
    StoreId(ObjectId),
    Store(Store),
}

impl DBModel for Product {
    fn get_collection_name() -> &'static str {
        "products"
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Product {
    pub fn new(
        name: String,
        store_id: ObjectId,
        description: String,
        sizes: Option<Vec<String>>,
        variants: Option<Vec<String>>,
    ) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            store: StoreField::StoreId(store_id),
            description,
            sizes: sizes.unwrap_or(Vec::new()),
            variants: variants.unwrap_or(Vec::new()),
        }
    }
}
