use super::common::{DBModel, db_model};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use bson::oid::ObjectId;
use crate::helpers::types::ResponseBuilder;
use axum::response::{Response, IntoResponse};
use mongodb::IndexModel;
#[derive(Serialize, Deserialize, Debug)]

pub struct SiteVisit {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub ip_address: String
}

impl DBModel for SiteVisit {
    fn get_collection_name() -> &'static str {
        "site_visits"
    }

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
    }

    db_model!(SiteVisit);
}

impl SiteVisit {
    pub fn new(ip_address: String) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            ip_address
        }
    }
}