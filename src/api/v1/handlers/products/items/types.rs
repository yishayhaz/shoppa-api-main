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
}
