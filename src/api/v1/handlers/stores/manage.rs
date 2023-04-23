use super::{super::prelude::routes::*, types};
use crate::db::{inserts, queries};
use axum::http::StatusCode;

pub async fn create_new_store(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResponse {
    let _ = inserts::new_store(&db, payload.name, payload.email, payload.location).await;

    Ok(ResponseBuilder::success(Some("dad"), None, None).into_response())
}

pub async fn get_stores(db: DBExtension, _: OnlyInDev) -> HandlerResponse {
    let stores = queries::get_stores(&db).await?;

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_store_by_id(
    db: DBExtension,
    _: OnlyInDev,
    Path(store_oid): Path<ObjectId>,
) -> HandlerResponse {
    let store = queries::get_store_by_id(&db, &store_oid).await?;

    
    Ok(ResponseBuilder::success(Some(store), None, None).into_response())
}
