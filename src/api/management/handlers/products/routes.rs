use super::types::{CreateProductPayload, UploadProductImagePayload};
use crate::{
    db::{
        inserts,
        models::{FileDocument, FileTypes},
        queries, updates,
    },
    prelude::{
        handlers::{DBExtension, StorgeClientExtension},
        *,
    },
    services::file_storage::upload_product_image,
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{models::Product, DBConection},
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn create_new_product(
    db: DBExtension,
    new_db: Extension<DBConection>,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let categories = queries::get_category_hierarchy_for_subsubcategory(
        &db,
        &payload.categories[0],
        &payload.categories[1],
        &payload.categories[2],
    )
    .await?;

    if categories.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let categories = categories.unwrap();

    let store = new_db.get_store_by_id(&payload.store, None, None).await?;

    if store.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let store = store.unwrap();

    let inner_category = categories.categories[0];

    let new_product = Product::new(
        &store,
        payload.brand,
        payload.description,
        payload.keywords.unwrap_or(vec![]),
        payload.name,
        &categories,
        &inner_category,
        &inner_category.categories[0],
        payload.variants.unwrap_or(vec![]),
    )?;

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}

pub async fn upload_product_images(
    db: DBExtension,
    storage_client: StorgeClientExtension,
    Path(product_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<UploadProductImagePayload>,
) -> HandlerResult {
    let product = queries::get_product_by_id(&db, &product_id, None, None).await?;

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

    updates::add_image_to_product(
        &db,
        &product_id,
        FileDocument::new(
            true,
            image.file_name,
            upload.key.clone(),
            image.size as u64,
            image.content_type.clone(),
            FileTypes::Image,
        ),
        None,
    )
    .await?;

    upload.fire(&storage_client).await;

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}
