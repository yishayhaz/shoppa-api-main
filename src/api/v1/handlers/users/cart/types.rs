use crate::prelude::types::*;


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

