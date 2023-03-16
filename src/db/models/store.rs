use super::common::{DBModel, db_model};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;
use crate::helpers::types::ResponseBuilder;
use axum::response::{Response, IntoResponse};
#[derive(Serialize, Deserialize, Debug)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
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
    db_model!(Store);
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