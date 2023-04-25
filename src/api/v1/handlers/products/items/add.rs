use super::types;
use crate::{
    db::{populate::ProductsPopulate, queries, updates},
    prelude::{handlers::*},
};

pub async fn add_product_item(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::AddProductItemPayload>,
) -> HandlerResponse {
    let populate = ProductsPopulate {
        store: false,
        categories: FieldPopulate::None,
        variants: true,
    };

    let product = queries::get_product_by_id(&db, &product_id, Some(populate), None).await?;

    if product.is_none() {
        return Err(ResponseBuilder::not_found_error("product", &product_id).into_response());
    };

    let mut product = product.unwrap();

    let new_item = match product.add_item(payload.price, payload.in_storage, payload.variants) {
        Ok(item) => item,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                "",
                None,
                Some("Some of the variants are not in the product variants!"),
                Some(400),
            )
            .into_response())
        }
    };

    updates::add_product_item(&db, &product_id, &new_item, None).await?;

    Ok(().into_response())
}
