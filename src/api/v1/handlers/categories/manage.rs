use argon2::Params;

use super::types::{
    CreateInnerCatgoriePayload, CreateInnerInnerCatgoriePayload, CreateRootCatgoriePayload, DeleteCategory, UpdateCategoryInfo
};
use crate::{
    api::v1::middlewares::*,
    db::{inserts, queries},
    prelude::{handlers::*, *},
};

pub async fn create_new_root_catagorie(
    db: DBExtension,
    _: OnlyInDev,
    Json(payload): Json<CreateRootCatgoriePayload>,
) -> HandlerResult {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, &variants_ids).await? {
            return Ok(ResponseBuilder::<u16>::error(
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
) -> HandlerResult {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, variants_ids).await? {
            return Ok(ResponseBuilder::<u16>::error(
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
) -> HandlerResult {
    if let Some(variants_ids) = &payload.variants {
        if !queries::validate_many_variants_exist(&db, &variants_ids).await? {
            return Ok(ResponseBuilder::<u16>::error(
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

pub async fn update_category_by_ids(
    db: DBExtension,
    _: OnlyInDev,
    payload: Json<UpdateCategoryInfo>,
) -> HandlerResult {
    let _ = queries::update_category_by_ids(&db, &payload.category_ids, &payload.name, &payload.variants).await?;

    Ok(().into_response())
}

pub async fn delete_category_by_ids(
    db: DBExtension,
    _: OnlyInDev,
    // read ids from params
    Path(payload): Path<DeleteCategory>,
) -> HandlerResult {
    let _ = queries::delete_category_by_ids(&db, &payload.category_ids).await?;

    Ok(().into_response())
}