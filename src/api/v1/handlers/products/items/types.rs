use crate::{
    db::models::ItemVariants,
    prelude::{types::*},
};

#[derive(Deserialize, Validate)]
pub struct AddProductItemPayload {
    // the store will come from the product id in the url
    #[validate(range(min = 13.98))]
    pub price: f64,
    pub in_storage: u64,
    pub variants: Vec<ItemVariants>,
    pub name: Option<String>,
    #[serde(default)]
    pub images_refs: Vec<ObjectId>,
}


#[derive(Deserialize, Validate)]
pub struct EditProductItemPayload {
    // the store will come from the product id in the url
    #[validate(range(min = 13.98))]
    pub price: Option<f64>,
    pub in_storage: Option<u64>,
    pub name: Option<String>,
    pub images_refs: Option<Vec<ObjectId>>,
}
