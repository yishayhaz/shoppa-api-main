use super::{super::super::prelude::routes::*, types};
use crate::db::{queries, populate::{ProductsPopulate, FieldPopulate}};

pub async fn add_product_item(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::AddProductItemPayload>
){

    let populate = ProductsPopulate{
        store: false,
        categories: FieldPopulate::None,
        variants: true,
    };

    let product = queries::get_product_by_id(&db, &product_id, Some(populate), None).await;
    // TODO get product by id
    // make sure that the length of the variants is the same
    // save new product item
    tracing::debug!("{:#?}", product);

}