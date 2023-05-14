use super::common::{db_model, DBModel, FileDocument};
mod fields;
use crate::{
    helpers::validators::{number_string_validator, phone_number_validator},
    prelude::{db_models::*, *},
};

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
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoreContact {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "phone_number_validator")]
    pub tel: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct StoreLocation {
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
    #[validate]
    pub legal_information: StoreLegalInformation,
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
    pub voters: u64,
    pub average: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct StoreLegalInformation {
    pub legal_id: String,
    pub business_type: StoreBusinessType,
    pub name: String,
}

// #[derive(Deserialize, Debug, Clone, PartialEq, EnumString)]
pub enum StoreBusinessType {
    ExemptDealer, // עוסק פטור
    AuthorizedDealer, // עוסק מורשה
    Ltd, // חברה בע"מ
    Public, // חברה ציבורית
    NonProfit, // מלכ"ר - עמותה
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
            voters: 0,
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
            banner: None,
            logo: None,
            slogan: None,
            contact: StoreContact {
                email,
                tel: String::new(),
            },
            analytics: StoreAnalytics::default(),
            locations: Vec::new(),
        }
    }

    pub fn fields() -> &'static fields::StoreFields {
        &fields::FIELDS
    }
}
