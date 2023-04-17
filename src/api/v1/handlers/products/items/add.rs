use super::{super::super::prelude::routes::*, types};
use crate::db::queries;

pub async fn add_product_item(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::AddProductItemPayload>
){

    // TODO get product by id
    // make sure that the length of the variants is the same
    // save new product item

}