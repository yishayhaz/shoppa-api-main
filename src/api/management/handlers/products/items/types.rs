use crate::{prelude::types::*};
use shoppa_core::db::models::ItemVariants;

#[derive(Deserialize, Validate)]
pub struct AddProductItemPayload {
    // the store will come from the product
    #[validate(range(min = 13.98))]
    pub price: f64,
    pub in_storage: u64,
    #[serde(default)]
    pub variants: Vec<ItemVariants>,
    pub name: Option<String>,
    #[serde(default)]
    pub assets_refs: Vec<ObjectId>,
}

#[derive(Deserialize, Validate)]
pub struct EditProductItemPayload {
    // the store will come from the product id in the url
    #[validate(range(min = 13.98))]
    pub price: Option<f64>,
    pub in_storage: Option<u64>,
    pub name: Option<String>,
    pub assets_refs: Option<Vec<ObjectId>>,
}
