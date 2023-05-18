use super::{
    common::{db_model, DBModel, FileDocument},
    constans,
    schame::{BsonType, MongoSchame},
};
mod fields;

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

    #[validate(length(
        min = "constans::STORE_NAME_MIN_LENGTH",
        max = "constans::STORE_NAME_MAX_LENGTH"
    ))]
    pub name: String,
    #[validate(length(
        min = "constans::STORE_DESCRIPTION_MIN_LENGTH",
        max = "constans::STORE_DESCRIPTION_MAX_LENGTH"
    ))]
    pub description: String,
    #[validate(length(
        min = "constans::STORE_SLOGAN_MIN_LENGTH",
        max = "constans::STORE_SLOGAN_MAX_LENGTH"
    ))]
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
    #[validate(length(max = "constans::LOCATION_FREE_TEXT_MAX_LENGTH"))]
    pub free_text: Option<String>,
    // 85 is the max length of a city name in the world
    #[validate(length(
        min = "constans::CITY_NAME_MIN_LENGTH",
        max = "constans::CITY_NAME_MAX_LENGTH"
    ))]
    pub city: String,
    #[validate(length(
        min = "constans::STREET_NAME_MIN_LENGTH",
        max = "constans::STREET_NAME_MAX_LENGTH"
    ))]
    pub street: String,
    #[validate(length(
        min = "constans::STREET_NUMBER_MIN_LENGTH",
        max = "constans::STREET_NUMBER_MAX_LENGTH"
    ))]
    pub street_number: String,
    #[validate(custom = "number_string_validator")]
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
        let builder = MongoSchame::builder();

        builder
            .bson_type(BsonType::Document)
            .add_defaults_to_schame()
            // name
            .add_property((
                Self::fields().name,
                MongoSchame::builder()
                    .bson_type(BsonType::String)
                    .min_length(constans::STORE_NAME_MIN_LENGTH)
                    .max_length(constans::STORE_NAME_MAX_LENGTH)
                    .build(),
            ))
            // description
            .add_property((
                Self::fields().description,
                MongoSchame::builder()
                    .bson_type(BsonType::String)
                    .min_length(constans::STORE_DESCRIPTION_MIN_LENGTH)
                    .max_length(constans::STORE_DESCRIPTION_MAX_LENGTH)
                    .build(),
            ))
            // slogan
            .add_property((
                Self::fields().slogan,
                MongoSchame::builder()
                    .add_bson_type(BsonType::String)
                    .add_bson_type(BsonType::Null)
                    .min_length(constans::STORE_SLOGAN_MIN_LENGTH)
                    .max_length(constans::STORE_SLOGAN_MAX_LENGTH)
                    .build(),
            ))
            // contact
            .add_property((
                Self::fields().contact,
                MongoSchame::builder()
                    .bson_type(BsonType::Document)
                    // email
                    .add_property((
                        Self::fields().contact(false).email,
                        MongoSchame::builder()
                            .bson_type(BsonType::String)
                            .pattern(constans::EMAIL_REGEX)
                            .build(),
                    ))
                    // phone
                    .add_property((
                        Self::fields().contact(false).phone,
                        MongoSchame::builder()
                            .bson_type(BsonType::String)
                            .pattern(constans::PHONE_REGEX)
                            .build(),
                    ))
                    .require_all_properties()
                    .build(),
            ))
            // locations
            .add_property((
                Self::fields().locations,
                MongoSchame::builder()
                    .bson_type(BsonType::Array)
                    .items(
                        MongoSchame::builder()
                            .add_bson_type(BsonType::Document)
                            // id
                            .add_property((
                                Self::fields().locations(false).id,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::ObjectId)
                                    .build(),
                            ))
                            // free_text
                            .add_property((
                                Self::fields().locations(false).free_text,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::String)
                                    .add_bson_type(BsonType::Null)
                                    .max_length(constans::LOCATION_FREE_TEXT_MAX_LENGTH)
                                    .build(),
                            ))
                            // city
                            .add_property((
                                Self::fields().locations(false).city,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::String)
                                    .min_length(constans::CITY_NAME_MIN_LENGTH)
                                    .max_length(constans::CITY_NAME_MAX_LENGTH)
                                    .build(),
                            ))
                            // street
                            .add_property((
                                Self::fields().locations(false).street,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::String)
                                    .min_length(constans::STREET_NAME_MIN_LENGTH)
                                    .max_length(constans::STREET_NAME_MAX_LENGTH)
                                    .build(),
                            ))
                            // street_number
                            .add_property((
                                Self::fields().locations(false).street_number,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::String)
                                    .min_length(constans::STREET_NUMBER_MIN_LENGTH)
                                    .max_length(constans::STREET_NUMBER_MAX_LENGTH)
                                    .build(),
                            ))
                            // phone
                            .add_property((
                                Self::fields().locations(false).phone,
                                MongoSchame::builder()
                                    .add_bson_type(BsonType::String)
                                    .pattern(constans::NUMBER_STRING_REGEX)
                                    .build(),
                            ))
                            .require_all_properties()
                            .build(),
                    )
                    .build(),
            ))
            // banner
            .file_properties(Self::fields().banner, true)
            // logo
            .file_properties(Self::fields().logo, true)
            // analytics
            .add_property((
                Self::fields().analytics,
                MongoSchame::builder()
                    .bson_type(BsonType::Document)
                    // views
                    .add_property((
                        Self::fields().analytics(false).views,
                        MongoSchame::builder()
                            .bson_type(BsonType::Int64)
                            .minimum(0)
                            .build(),
                    ))
                    // sales
                    .add_property((
                        Self::fields().analytics(false).sales,
                        MongoSchame::builder()
                            .bson_type(BsonType::Double)
                            .minimum(0)
                            .build(),
                    ))
                    // rating
                    .add_property((
                        Self::fields().analytics(false).rating,
                        MongoSchame::builder()
                            .bson_type(BsonType::Document)
                            // votes
                            .add_property((
                                Self::fields().analytics(false).rating(false).votes,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Int64)
                                    .minimum(0)
                                    .build(),
                            ))
                            // average
                            .add_property((
                                Self::fields().analytics(false).rating(false).average,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Double)
                                    .minimum(0)
                                    .build(),
                            ))
                            .require_all_properties()
                            .build(),
                    ))
                    // orders
                    .add_property((
                        Self::fields().analytics(false).orders,
                        MongoSchame::builder()
                            .bson_type(BsonType::Document)
                            // pending
                            .add_property((
                                Self::fields().analytics(false).orders(false).pending,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Int64)
                                    .minimum(0)
                                    .build(),
                            ))
                            // in_progress
                            .add_property((
                                Self::fields().analytics(false).orders(false).in_progress,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Int64)
                                    .minimum(0)
                                    .build(),
                            ))
                            // failed
                            .add_property((
                                Self::fields().analytics(false).orders(false).failed,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Int64)
                                    .minimum(0)
                                    .build(),
                            ))
                            // arrived
                            .add_property((
                                Self::fields().analytics(false).orders(false).arrived,
                                MongoSchame::builder()
                                    .bson_type(BsonType::Int64)
                                    .minimum(0)
                                    .build(),
                            ))
                            .require_all_properties()
                            .build(),
                    ))
                    .build(),
            ))
            // legal_information
            .add_property((
                Self::fields().legal_information,
                MongoSchame::builder()
                    .bson_type(BsonType::Document)
                    // legal_id
                    .add_property((
                        Self::fields().legal_information(false).legal_id,
                        MongoSchame::builder().bson_type(BsonType::String).build(),
                    ))
                    // business_type
                    .add_property((
                        Self::fields().legal_information(false).business_type,
                        MongoSchame::builder()
                            .bson_type(BsonType::String)
                            .enum_(vec![
                                "authorized_dealer",
                                "exempt_dealer",
                                "ltd",
                                "non_profit",
                                "public",
                            ])
                            .build(),
                    ))
                    // name
                    .add_property((
                        Self::fields().legal_information(false).name,
                        MongoSchame::builder().bson_type(BsonType::String).build(),
                    ))
                    .require_all_properties()
                    .build(),
            ));

        // pub analytics: StoreAnalytics,
        // #[validate]
        // pub legal_information: StoreLegalInformation,
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
