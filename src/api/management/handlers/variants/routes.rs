use super::types;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{extract::Path, response::IntoResponse};
use bson::oid::ObjectId;
use shoppa_core::{
    db::Pagination,
    extractors::{JsonWithValidation, QueryWithValidation},
    ResponseBuilder,
};

pub async fn get_variants(db: AxumDBExtansion, pagination: Pagination) -> HandlerResult {
    todo!();
    // let variants = queries::get_variants_for_extarnel(&db, Some(pagination)).await?;

    // Ok(ResponseBuilder::paginated_response(&variants).into_response())
}

pub async fn get_variants_by_ids(
    db: AxumDBExtansion,
    QueryWithValidation(query): QueryWithValidation<types::GetVariantsByIdsQuery>,
) -> HandlerResult {
    todo!("validate variants ids");
    // let variants = db.get_variants_by_ids(&db, &query.variants_ids).await?;

    // Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}

pub async fn get_variant_by_id(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    todo!("validate variant id");
    // let variant = queries::get_variant_by_id(&db, &variant_id).await?;

    // Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}

pub async fn create_new_variant(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::CreateVariantPayload>,
) -> HandlerResult {
    let variant = db.insert_new_variant(payload, None).await?;

    Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}

pub async fn update_variant(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantBasicInfoPayload>,
) -> HandlerResult {
    todo!();
    // let _ =
    //     updates::update_variant_basic_info(&db, &variant_id, &payload.name, &payload.type_).await;

    // Ok(().into_response())
}

pub async fn add_value_to_variant(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::CreateVariantValuePayload>,
) -> HandlerResult {
    todo!()
    // let _ = inserts::add_variant_value(&db, &variant_id, &payload.label, &payload.value).await;

    // Ok(().into_response())
}

pub async fn update_variant_value(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
    Path(value_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantValuePayload>,
) -> HandlerResult {
    todo!()
    // let _ =
    //     updates::update_variant_value(&db, &variant_id, &value_id, &payload.label, &payload.value)
    //         .await;

    // Ok(().into_response())
}

pub async fn delete_variant(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    // TODO: omer
    // 1. check if any category uses it
    // 2. if so - return error

    Ok(().into_response())
}

pub async fn delete_variant_value(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
    Path(value_id): Path<ObjectId>,
) -> HandlerResult {
    // TODO: omer

    // 1. check if any category uses `variant_id`
    // 2. if not -> go ahead and perform the delete
    // 3. if yes -> check if any product.variants.includes(variant_id) AND product.items[~].variants.value_id == `value_id`
    // 4. if yes -> return error
    // 5. if no -> go ahead and perform the delete

    Ok(().into_response())
}
