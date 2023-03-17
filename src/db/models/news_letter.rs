use super::common::{db_model, DBModel};
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use bson::{doc, oid::ObjectId};
use chrono::{DateTime, Utc};
use mongodb::{options::IndexOptions, IndexModel};
use serde::{Deserialize, Serialize, __private::doc};

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsLetterSubscriber {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub email: String,
}

impl DBModel for NewsLetterSubscriber {
    fn get_collection_name() -> &'static str {
        "news_letter_subscribers"
    }

    fn get_indexes() -> Vec<IndexModel> {
        let index1_option = IndexOptions::builder()
            .unique(true)
            .name(String::from("uniqe email"))
            .build();
        let index1 = IndexModel::builder()
            .keys(doc! {
                "email": 1
            })
            .options(index1_option)
            .build();

        vec![index1]
    }

    db_model!(NewsLetterSubscriber);
}

impl NewsLetterSubscriber {
    pub fn new(email: String) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            email,
        }
    }
}
