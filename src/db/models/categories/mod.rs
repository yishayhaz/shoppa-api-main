mod fields;

use super::{
    common::{db_model, embedded_document, DBModel, EmbeddedDocument, RefrenceField},
    prelude::*,
    Variants,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Categories {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub name: String,
    pub categories: Vec<InnerCategories>,
    pub allowed_variants: RefrenceField<Vec<Variants>, Vec<ObjectId>>,
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
    pub categories: Vec<InnerInnerCategories>,
    pub allowed_variants: RefrenceField<Vec<Variants>, Vec<ObjectId>>,
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
    pub allowed_variants: RefrenceField<Vec<Variants>, Vec<ObjectId>>,
}

impl DBModel for Categories {
    fn get_collection_name() -> &'static str {
        "categories"
    }

    fn get_indexes() -> Vec<IndexModel> {
        // TODO make name unique
        vec![]
    }

    db_model!(Categories);
}

impl EmbeddedDocument for InnerCategories {
    embedded_document!(InnerCategories);
}

impl EmbeddedDocument for InnerInnerCategories {
    embedded_document!(InnerInnerCategories);
}

impl Categories {
    pub fn new(
        name: String,
        // can be empty
        categories: Vec<InnerCategories>,
        allowed_variants: Option<Vec<ObjectId>>,
    ) -> Self {
        Self {
            id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            categories,
            allowed_variants: RefrenceField::NotPopulated(allowed_variants.unwrap_or(vec![])),
        }
    }

    pub fn fields() -> &'static fields::CategoriesFields {
        &fields::FIELDS
    }
}

impl InnerCategories {
    pub fn new(
        name: String,
        categories: Vec<InnerInnerCategories>,
        allowed_variants: Option<Vec<ObjectId>>,
    ) -> Self {
        Self {
            id: ObjectId::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            categories,
            allowed_variants: RefrenceField::NotPopulated(allowed_variants.unwrap_or(vec![])),
        }
    }
}

impl InnerInnerCategories {
    pub fn new(name: String, allowed_variants: Option<Vec<ObjectId>>) -> Self {
        Self {
            id: ObjectId::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name,
            allowed_variants: RefrenceField::NotPopulated(allowed_variants.unwrap_or(vec![])),
        }
    }
}
