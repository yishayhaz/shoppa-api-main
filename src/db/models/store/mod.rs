use super::common::{db_model, DBModel, FileDocument};
mod fields;
use crate::prelude::{db_models::*, *};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,        // TODO min: 4, max: 30
    pub description: String, // TODO min: 40, max: 110
    pub email: String,
    pub location: StoreLocation,
    pub banner: Option<FileDocument>,
    pub logo: Option<FileDocument>,

    // pub delivery_strategy: String
    // pub bank_details: ?
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreLocation {
    pub free_text: String, // TODO: min: 0, max: 60
}

impl DBModel for Store {
    fn get_collection_name() -> &'static str {
        "stores"
    }

    fn get_indexes() -> Vec<IndexModel> {
        let unique_index_options = IndexOptions::builder()
            .unique(true)
            .name(String::from("unique_store_name"))
            .build();

        let uniqe_index = IndexModel::builder()
            .keys(doc! {
                "name": 1
            })
            .options(unique_index_options)
            .build();

        vec![uniqe_index]
    }

    db_model!(Store);
}

impl Store {
    pub fn new(name: String, description: String, email: String, location: String) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            description,
            email,
            location: StoreLocation {
                free_text: location,
            },
            banner: None,
            logo: None,
        }
    }

    pub fn fields() -> &'static fields::StoreFields {
        &fields::FIELDS
    }
}
