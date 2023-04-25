use super::types;
use crate::{
    api::v1::middlewares::*,
    db::{inserts, queries},
    prelude::{handlers::*, *},
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
