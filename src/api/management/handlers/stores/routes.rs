use super::types;
use crate::{
    db::{
        inserts,
        models::{constans::DELETE_FIELD_KEY_OPETATOR, FileDocument, FileTypes},
        queries, updates,
    },
    helpers::extractors::MultipartFormWithValidation,
    prelude::{handlers::*, *},
    services::file_storage,
};

pub async fn create_new_store(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResult {
    let store = inserts::new_store(&db, payload).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn get_stores(db: DBExtension) -> HandlerResult {
    let stores = queries::get_stores(&db).await?;

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_store_by_id(db: DBExtension, Path(store_oid): Path<ObjectId>) -> HandlerResult {
    let store = queries::get_store_by_id(&db, &store_oid).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn update_store_assets(
    db: DBExtension,
    storage_client: StorgeClientExtension,
    Path(store_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<
        types::UpdateStoreAssetsPayload,
    >,
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

    let logo_doc = if let Some(logo_doc) = logo_doc {
        Some(Some(logo_doc))
    } else {
        None
    };

    let banner_doc = if let Some(banner_doc) = banner_doc {
        Some(Some(banner_doc))
    } else {
        None
    };

    updates::update_store(
        &db, &store_id, logo_doc, banner_doc, None, None, None, None, None, None, None, None, None,
    )
    .await?;

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

pub async fn update_store(
    db: DBExtension,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateStorePayload>,
) -> HandlerResult {
    let slogan = if let Some(slogan) = payload.slogan {
        if slogan == DELETE_FIELD_KEY_OPETATOR {
            Some(None)
        } else {
            Some(Some(slogan))
        }
    } else {
        None
    };

    let store = updates::update_store(
        &db,
        &store_id,
        None,
        None,
        payload.name,
        payload.description,
        slogan,
        payload.contact_email,
        payload.contact_phone,
        payload.legal_id,
        payload.business_type,
        payload.business_name,
        None,
    )
    .await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn add_store_locations(
    db: DBExtension,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::StoreLocationPayload>,
) -> HandlerResult {
    let _ = updates::add_store_locations(&db, &store_id, &payload).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn delete_store_location(
    db: DBExtension,
    Path((store_id, location_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    let _ = updates::delete_store_location(&db, &store_id, &location_id).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
