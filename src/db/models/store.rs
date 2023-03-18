use super::common::{db_model, DBModel};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
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

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
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
