use super::types;
use crate::{
    db::{models::EmbeddedDocument, populate::ProductsPopulate, queries, updates},
    prelude::{handlers::*, *},
};
use mongodb::options::FindOneAndUpdateOptions;

pub async fn add_product_item(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::AddProductItemPayload>,
) -> HandlerResult {
    let populate = ProductsPopulate {
        store: false,
        categories: FieldPopulate::None,
        variants: true,
    };

    let product = queries::get_product_by_id(&db, &product_id, Some(populate), None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::not_found_error("product", &product_id).into_response());
    };

    let mut product = product.unwrap();

    let new_item = product.add_item(
        payload.price,
        payload.in_storage,
        payload.variants,
        payload.name,
        payload.images_refs,
    )?;

    updates::add_product_item(&db, &product_id, &new_item, None).await?;

    Ok(().into_response())
}

pub async fn edit_product_item(
    db: DBExtension,
    Path((product_id, item_id)): Path<(ObjectId, ObjectId)>,
    JsonWithValidation(payload): JsonWithValidation<types::EditProductItemPayload>,
) -> HandlerResult {
    let product = queries::get_product_by_id(&db, &product_id, None, None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::not_found_error("product", &product_id).into_response());
    };

    let product = product.unwrap();

    if !product.items.iter().any(|item| item.id() == &item_id) {
        return Ok(ResponseBuilder::not_found_error("product.item", &item_id).into_response());
    }

    if let Some(images) = &payload.images_refs {
        let images_ids = product
            .images
            .iter()
            .map(|i| i.id.clone())
            .collect::<Vec<_>>();

        if !images.iter().all(|i| images_ids.contains(i)) {
            // not all images are in the product
            return Err(Error::Static("TODO"));
        }
    }

    let options = FindOneAndUpdateOptions::builder()
        .return_document(Some(mongodb::options::ReturnDocument::After))
        .build();

    let prouct = updates::edit_product_item(
        &db,
        &product_id,
        &item_id,
        payload.price,
        payload.in_storage,
        payload.name,
        payload.images_refs,
        Some(options),
    )
    .await?;

    Ok(
        ResponseBuilder::success(prouct, Some("Product item edited successfully"), None)
            .into_response(),
    )
}
