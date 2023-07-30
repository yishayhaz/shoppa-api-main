use crate::prelude::types::*;
use shoppa_core::{db::models::CartItem, payments::types::CreditCard, validators};
use std::collections::HashMap;
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

#[derive(Deserialize, Serialize, Validate)]
pub struct PayCartPayload {
    pub address_id: ObjectId,
    #[validate(custom = "validators::phone_number_validator")]
    pub phone_number: String,
    #[validate(email)]
    pub email: String,
    #[validate]
    pub credit_card: CreditCard,
    pub card_holder_name: String,
    #[serde(default)]
    pub utms: HashMap<ObjectId, String>,
}

impl From<AddProductToCartPayload> for CartItem {
    fn from(payload: AddProductToCartPayload) -> Self {
        Self::new(payload.product_id, payload.item_id, payload.quantity)
    }
}
