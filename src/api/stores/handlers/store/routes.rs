use super::types;
use crate::{
    api::stores::middlewares::CurrentUser,
    db::{AxumDBExtansion, StoreUserStoreFunctions},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{
    db::models::{FileDocument, FileTypes},
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    parser::FieldPatch,
    ResponseBuilder,
};

pub async fn get_my_store(db: AxumDBExtansion, current_user: CurrentUser) -> HandlerResult {
    let store = db
        .get_store_by_id(&current_user.store_id, None, None, None)
        .await?;

    Ok(ResponseBuilder::success(store, None, None).into_response())
}

pub async fn update_store_assets(
    db: AxumDBExtansion,
    storage_client: AxumStorgeClientExtension,
    current_user: CurrentUser,
    MultipartFormWithValidation(payload): MultipartFormWithValidation<
        types::UpdateStoreAssetsPayload,
    >,
) -> HandlerResult {
    let store = db
        .get_store_by_id(&current_user.store_id, None, None, None)
        .await?;

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

    if let Some(mut logo) = payload.logo {
        let upload = storage_client.upload_store_logo(
            logo.file,
            &logo.content_type,
            &current_user.store_id,
            &mut logo.file_extension,
        );

        logo_doc = Some(FileDocument::new(
            true,
            logo.file_name,
            upload.clone_key(),
            logo.size as u64,
            logo.content_type.clone(),
            FileTypes::Image,
        ));

        upload.fire().await;

        if let Some(logo) = store.logo {
            delete_files.push(logo.path);
        }
    }

    if let Some(mut banner) = payload.banner {
        let upload = storage_client.upload_store_banner(
            banner.file,
            &banner.content_type,
            &current_user.store_id,
            &mut banner.file_extension,
        );

        banner_doc = Some(FileDocument::new(
            true,
            banner.file_name,
            upload.clone_key(),
            banner.size as u64,
            banner.content_type.clone(),
            FileTypes::Image,
        ));

        upload.fire().await;

        if let Some(banner) = store.banner {
            delete_files.push(banner.path);
        }
    }

    db.update_store_base_data(
        &current_user.store_id,
        logo_doc,
        banner_doc,
        None,
        FieldPatch::Missing,
        None,
        None,
        None,
        None,
    )
    .await?;

    if delete_files.len() > 0 {
        tokio::spawn(async move {
            storage_client.delete_files(delete_files).await;
        });
    }

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn update_store(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateStorePayload>,
) -> HandlerResult {
    let store = db
        .update_store_base_data(
            &current_user.store_id,
            None,
            None,
            payload.description,
            payload.slogan,
            payload.contact_email,
            payload.contact_phone,
            payload.min_order,
            None,
        )
        .await?;

    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}

pub async fn add_store_locations(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    JsonWithValidation(payload): JsonWithValidation<types::StoreLocationPayload>,
) -> HandlerResult {
    let store = db
        .add_store_location(&current_user.store_id, &payload, None)
        .await?;

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
    current_user: CurrentUser,
    Path(location_id): Path<ObjectId>,
) -> HandlerResult {
    let store = db
        .delete_store_location(&current_user.store_id, &location_id, None)
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
    current_user: CurrentUser,
    Path(location_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateStoreLocationPayload>,
) -> HandlerResult {
    let store = db
        .update_store_location(
            &current_user.store_id,
            &location_id,
            &payload.city,
            &payload.street,
            &payload.street_number,
            payload.free_text,
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
