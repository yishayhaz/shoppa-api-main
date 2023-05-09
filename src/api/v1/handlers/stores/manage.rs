use super::types;
use crate::{
    api::v1::middlewares::*,
    db::{
        inserts,
        models::{FileDocument, FileTypes},
        queries, updates,
    },
    helpers::extractors::MultipartFrom,
    prelude::{handlers::*, *},
    services::file_storage,
};

pub async fn create_new_store(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResult {
    let _ = inserts::new_store(
        &db,
        payload.name,
        payload.description,
        payload.email,
        payload.location,
    )
    .await;

    Ok(ResponseBuilder::success(Some(""), None, None).into_response())
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
    MultipartFrom(payload): MultipartFrom<types::UpdateStorePayload>,
) -> HandlerResult {
    let store = queries::get_store_by_id(&db, &store_id).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    let mut logo_doc: Option<FileDocument> = None;
    let mut banner_doc: Option<FileDocument> = None;
    // TODO in the future, first update the db with the new data using a transaction and then upload the files
    // TODO if the upload fails, rollback the transaction
    if let Some(logo) = payload.logo {
        let res = file_storage::upload_store_logo(
            &storage_client,
            logo.file,
            &logo.content_type,
            &store_id,
            &logo.file_extension,
        )
        .await;

        logo_doc = Some(FileDocument::new(
            true,
            logo.file_name,
            res,
            logo.size as u64,
            logo.content_type,
            FileTypes::Image,
        ))
    }

    if let Some(banner) = payload.banner {
        let res = file_storage::upload_store_banner(
            &storage_client,
            banner.file,
            &banner.content_type,
            &store_id,
            &banner.file_extension,
        )
        .await;

        banner_doc = Some(FileDocument::new(
            true,
            banner.file_name,
            res,
            banner.size as u64,
            banner.content_type,
            FileTypes::Image,
        ))
    }

    updates::update_store(&db, &store_id, logo_doc, banner_doc, None).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
