use super::super::super::middlewares::CurrentUser;
use super::types::{
    CreateProductPayload, EditProductPayload, GetProductsQueryParams, UploadProductAssetPayload,
};
use crate::{
    db::{AdminProductFunctions, AxumDBExtansion, ProductSortBy},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{
        models::{EmbeddedDocument, FileDocument, FileTypes, Product, ProductStatus},
        populate::{FieldPopulate, ProductsPopulate},
        OptionalSorter, Pagination,
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn create_new_product(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let store = db.get_store_by_id(&current_user.store_id, None, None, None).await?;
    
    // TODO delete user cookie if store not found
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

pub async fn upload_product_asset(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    storage_client: AxumStorgeClientExtension,
    Path(product_id): Path<ObjectId>,
    MultipartFormWithValidation(mut payload): MultipartFormWithValidation<UploadProductAssetPayload>,
) -> HandlerResult {
    // once user successfully upload file, change status for productId to "pending"

    let product = db.get_product_by_id(&product_id, None, None, None).await?;

    if product.is_none() || product.unwrap().store_id() != &current_user.store_id {
        return Ok(ResponseBuilder::<()>::success(None, None, None).into_response());
    }

    let upload = storage_client.upload_product_image(
        payload.file.file,
        &payload.file.content_type,
        &product_id,
        &mut payload.file.file_extension,
    );

    let asset = FileDocument::new(
        true,
        payload.file.file_name,
        upload.clone_key(),
        payload.file.size as u64,
        payload.file.content_type.clone(),
        FileTypes::Image,
    );

    db.add_asset_to_product(&product_id, &asset, None, None)
        .await?;

    upload.fire().await;

    // Ok(ResponseBuilder::success(Some(asset), None, None).into_response())

    todo!()
}

pub async fn delete_product_asset(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    storage_client: AxumStorgeClientExtension,
    Path((product_id, file_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    todo!()
}

pub async fn edit_product(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(product_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<EditProductPayload>,
) -> HandlerResult {
    todo!()
}

pub async fn delete_product(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    todo!()
}

pub async fn get_product(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    todo!()
}

pub async fn get_products(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    pagination: Pagination,
    OptionalSorter(sorting): OptionalSorter<ProductSortBy>,
    Query(query): Query<GetProductsQueryParams>,
) -> HandlerResult {
    todo!()
}
