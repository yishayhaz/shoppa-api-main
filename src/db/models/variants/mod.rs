mod fields;

use super::{
    common::{db_model, embedded_document, DBModel, EmbeddedDocument},
    prelude::*,
};


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
        let unique_options = IndexOptions::builder()
            .name("unique_variant_name".to_string())
            .unique(true)
            .build();

        let unique_variant_name = IndexModel::builder()
            .keys(doc! {"name": 1})
            .options(unique_options)
            .build();

        vec![unique_variant_name]
    }

    db_model!(Categories);
}

impl EmbeddedDocument for VariantValue {
    embedded_document!(VariantValue);
}

impl Into<Bson> for VariantValue {
    fn into(self) -> bson::Bson {
        bson::Bson::Document(doc! {
            "_id": self.id,
            "created_at": self.created_at,
            "updated_at": self.updated_at,
            "name": self.name
        })
    }
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

    pub fn fields() -> &'static fields::VariantsFields {
        &fields::FIELDS
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
