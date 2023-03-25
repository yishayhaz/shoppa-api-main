use super::{
    common::{db_model, DBModel, RefrenceField},
    prelude::*,
    Categories, Store, Variants,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub brand: Option<String>,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub store: RefrenceField<Store, StoreField>,
    // Not likely that it will be populated.
    pub categories: RefrenceField<Categories, Vec<CategoriesField>>,
    pub variants: RefrenceField<Vec<Variants>, Vec<ObjectId>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CategoriesField {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoreField {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
}

impl DBModel for Product {
    fn get_collection_name() -> &'static str {
        "products"
    }

    fn get_indexes() -> Vec<IndexModel> {
        // we can add language key for the product to imrpove the text index,
        // or to set the default to hebrew

        let text_index_options = IndexOptions::builder()
            .weights(doc! {
                "name": 200,
                "keywords": 100,
                "description": 100,
                "brand": 50,
                "categories.name": 50,
                "store.name": 20
            })
            .name(String::from("search_text_index"))
            .default_language(String::from("none"))
            .build();

        let text_index = IndexModel::builder()
            .keys(doc! {
                "name": "text",
                "description": "text",
                "categories.name": "text",
                "brand": "text",
                "keywords": "text",
                "store.name": "text"
            })
            .options(text_index_options)
            .build();

        let unique_index_options = IndexOptions::builder()
            .unique(true)
            .name(String::from("unique_product_for_store"))
            .build();

        let uniqe_index = IndexModel::builder()
            .keys(doc! {
                "name": 1,
                "store._id": 1
            })
            .options(unique_index_options)
            .build();

        vec![text_index, uniqe_index]
    }

    db_model!(Product);
}
