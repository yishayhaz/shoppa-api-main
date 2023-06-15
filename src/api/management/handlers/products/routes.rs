use super::types::{CreateProductPayload, EditProductPayload, UploadProductImagePayload};
use crate::{
    db::{AdminProductFunctions, AxumDBExtansion},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{
        models::{FileDocument, FileTypes, Product, ProductStatus},
        populate::{FieldPopulate, ProductsPopulate},
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn create_new_product(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let store = db.get_store_by_id(&payload.store, None, None, None).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), None).into_response(),
        );
    }

    let store = store.unwrap();

    let categories = db
        .get_nested_ids_categories(&payload.categories, None, None, None)
        .await?;

    let new_product = Product::new(
        &store,
        payload.brand,
        payload.description,
        payload.keywords,
        payload.name,
        &categories,
        payload.variants,
        payload.feature_bullet_points,
        payload.warranty,
        None,
    )?;

    let product = db.insert_new_product(new_product, None, None).await?;

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}

pub async fn upload_product_images(
    db: AxumDBExtansion,
    storage_client: AxumStorgeClientExtension,
    Path(product_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<UploadProductImagePayload>,
) -> HandlerResult {
    let product = db.get_product_by_id(&product_id, None, None, None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let mut image = payload.file;

    let upload = storage_client.upload_product_image(
        image.file,
        &image.content_type,
        &product_id,
        &mut image.file_extension,
    );

    let asset = FileDocument::new(
        true,
        image.file_name,
        upload.clone_key(),
        image.size as u64,
        image.content_type.clone(),
        FileTypes::Image,
    );

    db.add_asset_to_product(&product_id, &asset, None, None)
        .await?;

    upload.fire().await;

    Ok(ResponseBuilder::success(Some(asset), None, None).into_response())
}

pub async fn edit_product(
    db: AxumDBExtansion,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<EditProductPayload>,
) -> HandlerResult {
    let res = db
        .edit_product_by_id(
            &product_id,
            payload.name,
            payload.keywords,
            payload.brand,
            payload.description,
            payload.feature_bullet_points,
            payload.warranty,
            None,
            None,
        )
        .await?;

    if res.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("product not found"), Some(404))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(Some(res), None, None).into_response())
}

pub async fn delete_product(
    db: AxumDBExtansion,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    // TODO if the product in draft status, delete it, else change status to deleted
    let res = db
        .edit_product_by_id(
            &product_id,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(ProductStatus::Deleted),
            None,
        )
        .await?;

    if res.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("product not found"), Some(404))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(Some(res), None, None).into_response())
}

pub async fn get_product(db: AxumDBExtansion, Path(product_id): Path<ObjectId>) -> HandlerResult {
    let res = db
        .get_product_by_id(
            &product_id,
            None,
            Some(ProductsPopulate {
                store: true,
                categories: FieldPopulate::None,
                variants: true,
                options: None,
            }),
            None,
        )
        .await?;

    if res.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("product not found"), Some(404))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(Some(res), None, None).into_response())
}
