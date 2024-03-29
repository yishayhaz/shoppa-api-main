use super::types;
use crate::{
    db::{AdminVariantsFunctions, AxumDBExtansion},
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::Pagination,
    extractors::{JsonWithValidation, QueryWithValidation},
    ResponseBuilder,
};

pub async fn get_variants(db: AxumDBExtansion, pagination: Pagination) -> HandlerResult {
    let variants = db.get_variants_for_extarnel(Some(pagination)).await?;

    Ok(ResponseBuilder::paginated_response(&variants).into_response())
}

pub async fn get_variants_by_ids(
    db: AxumDBExtansion,
    QueryWithValidation(query): QueryWithValidation<types::GetVariantsByIdsQuery>,
) -> HandlerResult {
    let variants = db.get_variants_by_ids(&query.variants_ids).await?;

    Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}

pub async fn get_variant_by_id(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    let variant = db.get_variant_for_extarnel(&variant_id).await?;

    Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}

pub async fn create_new_variant(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::CreateVariantPayload>,
) -> HandlerResult {
    let variant = db.insert_new_variant(payload, None, None).await?;

    Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}

pub async fn update_variant(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantBasicInfoPayload>,
) -> HandlerResult {
    let variant = db
        .update_variant_basic(
            &variant_id,
            &payload.name,
            &payload.type_,
            &payload
                .new_values
                .map(|values| values.into_iter().map(|v| v.into()).collect()),
        )
        .await?;

    if variant.is_none() {
        return Ok(ResponseBuilder::<()>::error("", None, None, Some(404)).into_response());
    }

    Ok(ResponseBuilder::success(Some(variant), None, None).into_response())
}

pub async fn update_variant_value(
    db: AxumDBExtansion,
    Path((variant_id, value_id)): Path<(ObjectId, ObjectId)>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantValuePayload>,
) -> HandlerResult {
    let variant = db
        .update_variant_value(&variant_id, &value_id, payload.value, payload.label)
        .await?;

    if variant.is_none() {
        return Ok(ResponseBuilder::<()>::error("", None, None, Some(404)).into_response());
    }

    Ok(ResponseBuilder::success(Some(variant.unwrap()), None, None).into_response())
}

pub async fn delete_variant(
    db: AxumDBExtansion,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    if db.check_if_variant_is_in_use(&variant_id).await? {
        return Ok(
            ResponseBuilder::<()>::error("", None, Some("variant is in use"), Some(400))
                .into_response(),
        );
    }

    let variant = db
        .find_and_delete_variant_by_id(&variant_id, None, None)
        .await?;

    if variant.is_none() {
        return Ok(ResponseBuilder::<()>::error("", None, None, Some(404)).into_response());
    }

    Ok(ResponseBuilder::success(variant, None, None).into_response())
}

pub async fn delete_variant_value(
    db: AxumDBExtansion,
    Path((variant_id, value_id)): Path<(ObjectId, ObjectId)>,
) -> HandlerResult {
    let delete_res;

    if !db.check_if_variant_is_in_use(&variant_id).await? {
        delete_res = db.delete_variant_value(&variant_id, &value_id).await?;
    } else {
        if db
            .check_if_variant_value_is_in_use(&variant_id, &value_id)
            .await?
        {
            return Ok(ResponseBuilder::<()>::error(
                "",
                None,
                Some("variant value is in use"),
                Some(400),
            )
            .into_response());
        }

        delete_res = db.delete_variant_value(&variant_id, &value_id).await?;
    }

    if delete_res.is_none() {
        return Ok(ResponseBuilder::<()>::error("", None, None, Some(404)).into_response());
    }

    Ok(ResponseBuilder::success(delete_res, None, None).into_response())
}

pub async fn autocomplete_variants(
    db: AxumDBExtansion,
    pagination: Pagination,
    Query(query): Query<types::GetVariantsAutocompleteQuery>,
) -> HandlerResult {
    let variants = db
        .autocomplete_variants_search(Some(pagination), query.categories_ids, query.free_text)
        .await?;

    Ok(ResponseBuilder::success(Some(variants), None, None).into_response())
}
