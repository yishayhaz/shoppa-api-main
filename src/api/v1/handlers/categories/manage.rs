use super::super::prelude::routes::*;
use super::types::{
    CreateInnerCatgoriePayload, CreateInnerInnerCatgoriePayload, CreateRootCatgoriePayload,
};
use crate::db::inserts;

pub async fn create_new_root_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Json(payload): Json<CreateRootCatgoriePayload>,
) -> HandlerResponse {
    let _ = inserts::new_root_catagorie(&db, payload.name).await;

    Ok(().into_response())
}

pub async fn create_new_inner_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Path(parent_id): Path<ObjectId>,
    Json(payload): Json<CreateInnerCatgoriePayload>,
) -> HandlerResponse {
    let _ = inserts::new_inner_catagorie(&db, payload.name, &parent_id).await;

    Ok(().into_response())
}

pub async fn create_new_inner_inner_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Path((parent_parent_id, parent_id)): Path<(ObjectId, ObjectId)>,
    Json(payload): Json<CreateInnerInnerCatgoriePayload>,
) -> HandlerResponse {
    let _ = inserts::new_inner_inner_catagorie(
        &db,
        payload.name,
        &parent_parent_id,
        &parent_id,
    )
    .await;

    Ok(().into_response())
}
