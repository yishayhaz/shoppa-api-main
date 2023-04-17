use super::super::prelude::routes::*;
use super::types::{
    CreateInnerCatgoriePayload, CreateInnerInnerCatgoriePayload, CreateRootCatgoriePayload,
};
use crate::db::{inserts, queries};

pub async fn create_new_root_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Json(payload): Json<CreateRootCatgoriePayload>,
) -> HandlerResponse {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, &variants_ids).await? {
            return Err(ResponseBuilder::<u16>::error(
                "",
                None,
                Some("Some of the provided variants dont exist"),
                Some(404),
            )
            .into_response());
        }
    }

    let _ = inserts::new_root_catagorie(&db, payload.name, payload.variants).await;

    Ok(().into_response())
}

pub async fn create_new_inner_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Path(parent_id): Path<ObjectId>,
    Json(payload): Json<CreateInnerCatgoriePayload>,
) -> HandlerResponse {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, variants_ids).await? {
            return Err(ResponseBuilder::<u16>::error(
                "",
                None,
                Some("Some of the provided variants dont exist"),
                Some(404),
            )
            .into_response());
        }
    }

    let _ = inserts::new_inner_catagorie(&db, payload.name, &parent_id, payload.variants).await;

    Ok(().into_response())
}

pub async fn create_new_inner_inner_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Path((parent_parent_id, parent_id)): Path<(ObjectId, ObjectId)>,
    Json(payload): Json<CreateInnerInnerCatgoriePayload>,
) -> HandlerResponse {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, &variants_ids).await? {
            return Err(ResponseBuilder::<u16>::error(
                "",
                None,
                Some("Some of the provided variants dont exist"),
                Some(404),
            )
            .into_response());
        }
    }

    let _ = inserts::new_inner_inner_catagorie(
        &db,
        payload.name,
        &parent_parent_id,
        &parent_id,
        payload.variants,
    )
    .await;

    Ok(().into_response())
}
