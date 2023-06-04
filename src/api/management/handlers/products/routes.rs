use super::types::{CreateProductPayload, UploadProductImagePayload};
use crate::{
    db::{AdminProductFunctions, AxumDBExtansion, CategoriesFunctions},
    prelude::{handlers::StorgeClientExtension, *},
    services::file_storage::upload_product_image,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{
    db::models::{FileDocument, FileTypes, Product},
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn create_new_product(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let categories = db
        .get_nested_categories(
            &payload.categories[0],
            &payload.categories[1],
            &payload.categories[2],
            None,
        )
        .await?;

    if categories.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let (category, inner_category, inner_inner_category) = categories.unwrap();

    let store = db.get_store_by_id(&payload.store, None, None).await?;

    if store.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let store = store.unwrap();

    let new_product = Product::new(
        &store,
        payload.brand,
        payload.description,
        payload.keywords,
        payload.name,
        &category,
        &inner_category,
        &inner_inner_category,
        payload.variants,
        payload.feature_bullet_points,
    )?;

    let product = db.insert_new_product(new_product, None).await?;

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}

pub async fn upload_product_images(
    db: AxumDBExtansion,
    storage_client: StorgeClientExtension,
    Path(product_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<UploadProductImagePayload>,
) -> HandlerResult {
    let product = db.get_product_by_id(&product_id, None, None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let product = product.unwrap();

    let image = payload.file;

    let upload = upload_product_image(
        image.file,
        &image.content_type,
        &product_id,
        &image.file_extension,
    );

    let asset = FileDocument::new(
        true,
        image.file_name,
        upload.key.clone(),
        image.size as u64,
        image.content_type.clone(),
        FileTypes::Image,
    );

    db.add_asset_to_product(&product_id, &asset, None, None)
        .await?;

    upload.fire(&storage_client).await;

    Ok(ResponseBuilder::success(Some(asset), None, None).into_response())
}
