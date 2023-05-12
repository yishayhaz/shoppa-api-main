use super::types::{CreateProductPayload, UploadProductImagesPayload};
use crate::{
    api::v1::middlewares::OnlyInDev,
    db::{
        inserts,
        models::{FileDocument, FileTypes},
        queries, updates,
    },
    helpers::extractors::MultipartFrom,
    prelude::{handlers::*, *},
    services::file_storage::upload_product_image,
};

pub async fn create_new_product(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<CreateProductPayload>,
) -> HandlerResult {
    let categories = queries::get_category_hierarchy_for_subsubcategory(
        &db,
        // we can safely unwrap since the CreateProductPayload validate the length of the categories
        payload.categories.get(0).unwrap(),
        payload.categories.get(1).unwrap(),
        payload.categories.get(2).unwrap(),
    )
    .await?;

    if categories.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let categories = categories.unwrap();

    let store = queries::get_store_by_id(&db, &payload.store).await?;

    if store.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let store = store.unwrap();

    let inner_category = categories.categories.get(0).unwrap();

    let product = inserts::new_product(
        &db,
        &store,
        payload.brand,
        payload.description,
        payload.keywords.unwrap_or(vec![]),
        payload.name,
        &categories,
        inner_category,
        inner_category.categories.get(0).unwrap(),
        payload.variants.unwrap_or(vec![]),
    )
    .await?;

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}

pub async fn add_view_to_product(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    let product = updates::add_view_to_product(&db, &product_id, None).await?;

    if let Some(product) = product {
        // the views is being returend before the update, so we need to add 1 to the views
        return Ok(
            ResponseBuilder::success(Some(product.analytics.views + 1), None, None).into_response(),
        );
    }

    Ok(ResponseBuilder::not_found_error("product", &product_id).into_response())
}

pub async fn upload_product_images(
    db: DBExtension,
    storage_client: StorgeClientExtension,
    Path(product_id): Path<ObjectId>,
    _: OnlyInDev,
    MultipartFrom(payload): MultipartFrom<UploadProductImagesPayload>,
) -> HandlerResult {
    let product = queries::get_product_by_id(&db, &product_id, None, None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::<u16>::success(None, None, None).into_response());
    }

    let product = product.unwrap();

    for image in payload.files {
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
    }

    Ok(ResponseBuilder::success(Some(product), None, None).into_response())
}
