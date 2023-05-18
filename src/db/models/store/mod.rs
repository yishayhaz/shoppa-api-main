use super::common::{db_model, DBModel, FileDocument};
mod fields;
mod schame;

use crate::{
    helpers::validators::{number_string_validator, phone_number_validator},
    prelude::{db_models::*, *},
};
use bson::Document;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Store {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    #[validate(length(min = 3, max = 60))]
    pub name: String,
    #[validate(length(min = 20, max = 160))]
    pub description: String,
    #[validate(length(min = 8, max = 40))]
    pub slogan: Option<String>,
    #[validate]
    pub contact: StoreContact,
    #[validate]
    pub locations: Vec<StoreLocation>,
    pub banner: Option<FileDocument>,
    pub logo: Option<FileDocument>,
    pub analytics: StoreAnalytics,
    #[validate]
    pub legal_information: StoreLegalInformation,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoreContact {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "phone_number_validator")]
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoreLocation {
    #[serde(rename = "_id")]
    id: ObjectId,
    #[validate(length(max = 100))]
    pub free_text: Option<String>,
    // 85 is the max length of a city name in the world
    #[validate(length(min = 2, max = 85))]
    pub city: String,
    #[validate(length(min = 2, max = 85))]
    pub street: String,
    #[validate(length(min = 2, max = 85))]
    pub street_number: String,
    #[validate(length(min = 2, max = 12), custom = "number_string_validator")]
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StoreAnalytics {
    pub views: u64,
    pub sales: f64,
    pub rating: StoreRating,
    pub orders: StoreOrdersStats,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StoreOrdersStats {
    pub pending: u64,
    pub in_progress: u64,
    pub failed: u64,
    pub arrived: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StoreRating {
    pub votes: u64,
    pub average: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoreLegalInformation {
    pub legal_id: String,
    pub business_type: StoreBusinessType,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumString)]
#[serde(rename_all = "snake_case")]
pub enum StoreBusinessType {
    ExemptDealer,     // עוסק פטור
    AuthorizedDealer, // עוסק מורשה
    Ltd,              // חברה בע"מ
    Public,           // חברה ציבורית
    NonProfit,        // מלכ"ר - עמותה
}

impl Default for StoreAnalytics {
    fn default() -> Self {
        Self {
            views: 0,
            sales: 0.0,
            rating: StoreRating::default(),
            orders: StoreOrdersStats::default(),
        }
    }
}

impl Default for StoreOrdersStats {
    fn default() -> Self {
        Self {
            pending: 0,
            in_progress: 0,
            failed: 0,
            arrived: 0,
        }
    }
}

impl Default for StoreRating {
    fn default() -> Self {
        Self {
            votes: 0,
            average: 0.0,
        }
    }
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

    fn collection_validator() -> Option<Document> {
        None
    }

    db_model!(Store);
}

impl StoreLocation {
    pub fn new(
        city: String,
        street: String,
        street_number: String,
        phone: String,
        free_text: Option<String>,
    ) -> Self {
        Self {
            id: ObjectId::new(),
            city,
            street,
            street_number,
            phone,
            free_text,
        }
    }
}

impl Store {
    pub fn new(
        store_name: String,
        description: String,
        contact_email: String,
        contact_phone: String,
        slogan: Option<String>,
        legal_id: String,
        business_type: StoreBusinessType,
        legal_name: String,
    ) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),

            name: store_name,
            slogan,
            description,

            banner: None,
            logo: None,

            locations: Vec::new(),
            legal_information: StoreLegalInformation {
                legal_id,
                business_type,
                name: legal_name,
            },
            contact: StoreContact {
                email: contact_email,
                phone: contact_phone,
            },
            analytics: StoreAnalytics::default(),
        }
    }

    pub fn fields() -> &'static fields::StoreFields {
        &fields::FIELDS
    }
}
