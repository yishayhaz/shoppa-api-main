use super::types;
use crate::{
    db::{AdminStoreFunctions, AxumDBExtansion},
    prelude::{handlers::StorgeClientExtension, *},
    services::file_storage,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{
        models::{FileDocument, FileTypes},
        Pagination,
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn create_new_store(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResult {
    let store = db.insert_new_store(payload, None).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn get_store_by_id(db: AxumDBExtansion, Path(store_id): Path<ObjectId>) -> HandlerResult {
    let store = db.get_store_by_id(&store_id, None, None).await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn update_store_assets(
    db: AxumDBExtansion,
    storage_client: StorgeClientExtension,
    Path(store_id): Path<ObjectId>,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<
        types::UpdateStoreAssetsPayload,
    >,
) -> HandlerResult {
    let store = db.get_store_by_id(&store_id, None, None).await?;

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

        if let Some(logo) = store.logo {
            delete_files.push(logo.path);
        }
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

        if let Some(banner) = store.banner {
            delete_files.push(banner.path);
        }
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

    db.update_store_base_data(
        &store_id, logo_doc, banner_doc, None, None, None, None, None, None, None, None, None,
    )
    .await?;

    if delete_files.len() > 0 {
        tokio::spawn(async move {
            file_storage::delete_files(delete_files, &storage_client).await;
        });
    }

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn update_store(
    db: AxumDBExtansion,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateStorePayload>,
) -> HandlerResult {
    let store = db
        .update_store_base_data(
            &store_id,
            None,
            None,
            payload.name,
            payload.description,
            payload.slogan,
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
    db: AxumDBExtansion,
    Path(store_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::StoreLocationPayload>,
) -> HandlerResult {
    let store = db.add_store_location(&store_id, &payload, None).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(store, None, None).into_response())
}

pub async fn delete_store_location(
    db: AxumDBExtansion,
    Path((store_id, location_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    let store = db
        .delete_store_location(&store_id, &location_id, None)
        .await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(store, None, None).into_response())
}

pub async fn update_store_location(
    db: AxumDBExtansion,
    Path((store_id, location_id)): Path<(ObjectId, ObjectId)>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateStoreLocationPayload>,
) -> HandlerResult {
    let store = db
        .update_store_location(
            &store_id,
            &location_id,
            &payload.city,
            &payload.street,
            &payload.street_number,
            &payload.free_text,
            &payload.phone,
            None,
        )
        .await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(store, None, None).into_response())
}

pub async fn get_stores(db: AxumDBExtansion, pagination: Pagination) -> HandlerResult {
    let stores = db.get_stores_for_admins(Some(pagination), None).await?;

    Ok(ResponseBuilder::paginated_response(&stores).into_response())
}
