use super::common::{db_model, nested_document, DBModel, NestedDocument};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Categories {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
    // TODO method to get categries + method to add a new categorie
    categories: Vec<InnerCategories>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InnerCategories {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
    // TODO method to get categries + method to add a new categorie
    categories: Vec<InnerInnerCategories>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InnerInnerCategories {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,
    
    pub name: String,
}

impl DBModel for Categories {
    fn get_collection_name() -> &'static str {
        "categories"
    }

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
    }

    db_model!(Categories);
}

impl NestedDocument for InnerCategories {
    nested_document!(InnerCategories);
}

impl NestedDocument for InnerInnerCategories {
    nested_document!(InnerInnerCategories);
}
