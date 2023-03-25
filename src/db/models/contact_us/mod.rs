use super::{
    common::{db_model, DBModel},
    prelude::*,
};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
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

    fn get_indexes() -> Vec<IndexModel> {
        vec![]
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
