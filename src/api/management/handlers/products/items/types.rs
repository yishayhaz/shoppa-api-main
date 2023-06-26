use crate::prelude::types::*;
use shoppa_core::db::models::{ItemVariants, ProductItem};

#[derive(Deserialize, Validate)]
pub struct AddProductItemPayload {
    pub price: f64,
    pub in_storage: u64,
    #[serde(default)]
    pub variants: Vec<ItemVariants>,
    pub name: Option<String>,
    #[serde(default)]
    pub assets_refs: Vec<ObjectId>,
    pub info: Option<String>,
    pub sku: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct EditProductItemPayload {
    pub price: Option<f64>,
    pub in_storage: Option<u64>,
    pub name: Option<String>,
    pub assets_refs: Option<Vec<ObjectId>>,
    pub sku: Option<String>,
    pub info: Option<String>,
}

impl Into<ProductItem> for AddProductItemPayload {
    fn into(self) -> ProductItem {
        ProductItem::new(
            self.price,
            self.in_storage,
            self.variants,
            self.name,
            self.assets_refs,
            self.info,
            self.sku,
        )
    }
}
