use super::common::DBModel;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,
    
    pub name: String,
}

impl DBModel for Store {
    fn get_collection_name() -> &'static str {
        "stores"
    }
    fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl Store {
    pub fn new(name: String) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
        }
    }
}