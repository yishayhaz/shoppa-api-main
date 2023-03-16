use super::common::{DBModel, db_model};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContactUsForm {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub email: String,
    pub message: String,
    pub reason: ContactUsReason,
}

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ContactUsReason {
    General,
    FeatureRequest,
    GetToKnowUs,
}

impl DBModel for ContactUsForm {
    fn get_collection_name() -> &'static str {
        "contact_us_forms"
    }

    db_model!(ContactUsForm);
}

impl ContactUsForm {
    pub fn new(email: String, message: String, reason: ContactUsReason) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            email,
            message,
            reason,
        }
    }
}
