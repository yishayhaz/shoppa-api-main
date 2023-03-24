use super::common::{db_model, nested_document, DBModel, NestedDocument};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Variants {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
    pub values: Vec<VariantValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VariantValue {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
}

impl DBModel for Variants {
    fn get_collection_name() -> &'static str {
        "variants"
    }

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
    }

    db_model!(Categories);
}

impl NestedDocument for VariantValue {
    nested_document!(VariantValue);
}

impl Variants {
    pub fn new(name: String, values: Vec<VariantValue>) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            values,
        }
    }
}

impl VariantValue {
    pub fn new(name: String) -> Self {
        Self {
            id: ObjectId::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
        }
    }
}
