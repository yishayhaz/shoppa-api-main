use crate::prelude::types::*;
use shoppa_core::db::models::CartItem;

#[derive(Deserialize, Serialize, Validate)]
pub struct AddProductToCartPayload {
    pub product_id: ObjectId,
    pub item_id: ObjectId,
    #[validate(range(min = 1))]
    pub quantity: u32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct EditProductInCartPayload {
    pub product_id: ObjectId,
    pub item_id: ObjectId,
    pub new_quantity: u32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RemoveProductFromCartQuery {
    pub product_id: ObjectId,
    pub item_id: ObjectId,
}

impl From<AddProductToCartPayload> for CartItem {
    fn from(payload: AddProductToCartPayload) -> Self {
        Self::new(payload.product_id, payload.item_id, payload.quantity)
    }
}
