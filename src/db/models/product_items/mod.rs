use super::{
    common::{db_model, DBModel, RefrenceField},
    prelude::*,
    Product, Store,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProductItems {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    updated_at: DateTime<Utc>,

    pub product: RefrenceField<Product, ObjectId>,
    pub store: RefrenceField<Store, ObjectId>,
    pub price: f64,
    pub in_storage: u64,
    // this field describe the variant of the givem item: e.g: size L and color red.
    // so it will be uniqe with the product id to make sure there is no double items with
    // the same variants, the length of the variants field here need to be the same as the one in the parent product.
    pub variants: Vec<ItemVariants>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemVariants {
    // the variant _id field.
    pub variant_id: ObjectId,
    // the above variant value id in his values field.
    pub value_id: ObjectId,
}

impl DBModel for ProductItems {
    fn get_collection_name() -> &'static str {
        "product_items"
    }

    fn get_indexes() -> Vec<IndexModel> {
        let unique_index_options = IndexOptions::builder()
            .unique(true)
            .name(String::from("unique_item_variation"))
            .build();

        let uniqe_index = IndexModel::builder()
            .keys(doc! {
                "product": 1,
                "variants": 1
            })
            .options(unique_index_options)
            .build();

        vec![uniqe_index]
    }

    db_model!(ProductItems);
}
