use crate::prelude::types::*;
use shoppa_core::{
    constans,
    db::models::{ItemVariants, ProductItem, ProductItemStatus},
    parser::FieldPatch,
};

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
    #[serde(default)]
    #[validate(length(
        min = "constans::PRODUCT_NAME_MIN_LENGTH",
        max = "constans::PRODUCT_NAME_MAX_LENGTH"
    ))]
    pub name: FieldPatch<String>,
    pub assets_refs: Option<Vec<ObjectId>>,
    #[serde(default)]
    pub sku: FieldPatch<String>,
    #[serde(default)]
    pub info: FieldPatch<String>,
    pub status: Option<ProductItemStatus>,
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
