use super::types;
use crate::{
    db::{AdminProductFunctions, AxumDBExtansion},
    prelude::*,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use mongodb::options::FindOneAndUpdateOptions;
use shoppa_core::{
    db::{
        models::EmbeddedDocument,
        populate::{FieldPopulate, ProductsPopulate},
    },
    extractors::JsonWithValidation,
    ResponseBuilder,
};

pub async fn add_product_item(
    db: AxumDBExtansion,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::AddProductItemPayload>,
) -> HandlerResult {
    let populate = ProductsPopulate {
        store: false,
        categories: FieldPopulate::None,
        variants: true,
        options: None,
    };

    let product = db
        .get_product_by_id(&product_id, None, Some(populate), None)
        .await?;

    if product.is_none() {
        return Ok(
            ResponseBuilder::error("", Some(""), Some("product not found"), Some(404))
                .into_response(),
        );
    };

    let product = product.unwrap();

    db.add_item_to_product(&product, payload, None).await?;

    Ok(
        ResponseBuilder::success(None::<()>, Some("Product item added successfully"), None)
            .into_response(),
    )
}

pub async fn edit_product_item(
    db: AxumDBExtansion,
    Path((product_id, item_id)): Path<(ObjectId, ObjectId)>,
    JsonWithValidation(payload): JsonWithValidation<types::EditProductItemPayload>,
) -> HandlerResult {
    let product = db.get_product_by_id(&product_id, None, None, None).await?;

    if product.is_none() {
        return Ok(
            ResponseBuilder::error("", Some(""), Some("product not found"), Some(404))
                .into_response(),
        );
    };

    let product = product.unwrap();

    if !product.items.iter().any(|item| item.id() == &item_id) {
        return Ok(
            ResponseBuilder::error("", Some(""), Some("product item not found"), Some(404))
                .into_response(),
        );
    }

    if let Some(assets) = &payload.assets_refs {
        let assets_ids = product.assets.iter().map(|i| *i.id()).collect::<Vec<_>>();

        if !assets.iter().all(|i| assets_ids.contains(i)) {
            // not all images are in the product
            return Ok(ResponseBuilder::<()>::error(
                "",
                None,
                Some("some of the provided assets dont exists"),
                Some(404),
            )
            .into_response());
        }
    }

    let options = FindOneAndUpdateOptions::builder()
        .return_document(Some(mongodb::options::ReturnDocument::After))
        .build();

    let prouct = db
        .edit_product_item(
            &product_id,
            &item_id,
            payload.price,
            payload.in_storage,
            payload.name,
            payload.assets_refs,
            payload.sku,
            payload.info,
            payload.status,
            Some(options),
        )
        .await?;

    Ok(
        ResponseBuilder::success(prouct, Some("Product item edited successfully"), None)
            .into_response(),
    )
}

pub async fn delete_product_item(
    db: AxumDBExtansion,
    Path((product_id, item_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    let res = db.delete_product_item(&product_id, &item_id, None).await?;

    if res.matched_count == 0 {
        return Ok(
            ResponseBuilder::error("", Some(""), Some("product item not found"), Some(404))
                .into_response(),
        );
    }

    if res.modified_count == 0 {
        return Ok(ResponseBuilder::error(
            "",
            Some(""),
            Some("product item not deleted"),
            Some(500),
        )
        .into_response());
    }

    Ok(
        ResponseBuilder::<()>::success(None, Some("Product item deleted successfully"), None)
            .into_response(),
    )
}
