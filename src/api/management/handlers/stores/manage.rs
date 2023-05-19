use super::types;
use crate::{
    api::v1::middlewares::*,
    db::{
        inserts,
        models::{FileDocument, FileTypes},
        queries, updates,
    },
    helpers::extractors::MultipartFormWithValidation,
    prelude::{handlers::*, *},
    services::file_storage,
};

pub async fn create_new_store(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResult {
    let store = inserts::new_store(&db, payload).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn get_stores(db: DBExtension, _: OnlyInDev) -> HandlerResult {
    let stores = queries::get_stores(&db).await?;

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_store_by_id(
    db: DBExtension,
    _: OnlyInDev,
    Path(store_oid): Path<ObjectId>,
) -> HandlerResult {
    let store = queries::get_store_by_id(&db, &store_oid).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn get_stores_count(db: DBExtension, _: OnlyInDev) -> HandlerResult {
    let count = queries::get_stores_count(&db).await?;

    Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}

pub async fn update_store(
    db: DBExtension,
    storage_client: StorgeClientExtension,
    _: OnlyInDev,
    Path(store_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<types::UpdateStorePayload>,
) -> HandlerResult {
    let store = queries::get_store_by_id(&db, &store_id).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    let store = store.unwrap();

    let mut logo_doc: Option<FileDocument> = None;
    let mut banner_doc: Option<FileDocument> = None;
    // TODO in the future, first update the db with the new data using a transaction and then upload the files
    // TODO if the upload fails, rollback the transaction

    let mut delete_files: Vec<String> = Vec::new();

    if let Some(logo) = payload.logo {
        let upload = file_storage::upload_store_logo(
            logo.file,
            &logo.content_type,
            &store_id,
            &logo.file_extension,
        );

        logo_doc = Some(FileDocument::new(
            true,
            logo.file_name,
            upload.key.clone(),
            logo.size as u64,
            logo.content_type.clone(),
            FileTypes::Image,
        ));

        upload.fire(&storage_client).await;
    }

    if let Some(banner) = payload.banner {
        let upload = file_storage::upload_store_banner(
            banner.file,
            &banner.content_type,
            &store_id,
            &banner.file_extension,
        );

        banner_doc = Some(FileDocument::new(
            true,
            banner.file_name,
            upload.key.clone(),
            banner.size as u64,
            banner.content_type.clone(),
            FileTypes::Image,
        ));

        upload.fire(&storage_client).await;
    }

    updates::update_store(&db, &store_id, logo_doc, banner_doc, None).await?;

    if let Some(logo) = store.logo {
        delete_files.push(logo.path);
    }

    if let Some(banner) = store.banner {
        delete_files.push(banner.path);
    }

    if delete_files.len() > 0 {
        tokio::spawn(async move {
            file_storage::delete_files(delete_files, &storage_client).await;
        });
    }

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn update_store_location(
    db: DBExtension,
    _: OnlyInDev,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(location): JsonWithValidation<types::StoreLocationPayload>,
) -> HandlerResult {
    let _ = updates::update_store_location(&db, &store_id, &location).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn add_store_location(
    db: DBExtension,
    _: OnlyInDev,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(location): JsonWithValidation<types::StoreLocationPayload>,
) -> HandlerResult {
    let _ = inserts::add_store_location(&db, &store_id, &location).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn delete_store_location(
    db: DBExtension,
    _: OnlyInDev,
    Path(store_id): Path<ObjectId>,
    Path(location_id): Path<ObjectId>,
) -> HandlerResult {
    let _ = updates::delete_store_location(&db, &store_id, &location_id).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}