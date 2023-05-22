use super::types;
use crate::{
    api::v1::middlewares::*,
    db::{inserts, updates},
    prelude::{handlers::*, *},
};

pub async fn create_new_variant(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<types::CreateVariantPayload>,
) -> HandlerResult {
    let _ = inserts::new_variant(&db, payload.name, payload.values, payload.type_).await;

    Ok(().into_response())
}

pub async fn update_variant(
    db: DBExtension,
    _: OnlyInDev,
    Path(variant_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantBasicInfoPayload>,
) -> HandlerResult {
    let _ = updates::update_variant_basic_info(&db, &variant_id, &payload.name, &payload.type_).await;
    
    Ok(().into_response())
}


pub async fn add_value_to_variant(
    db: DBExtension,
    _: OnlyInDev,
    Path(variant_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::CreateVariantValuePayload>,
) -> HandlerResult {
    let _ = inserts::add_variant_value(&db, &variant_id, &payload.label, &payload.value).await;

    Ok(().into_response())
}

pub async fn update_variant_value(
    db: DBExtension,
    _: OnlyInDev,
    Path(variant_id): Path<ObjectId>,
    Path(value_id): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateVariantValuePayload>,
) -> HandlerResult {
    let _ = updates::update_variant_value(&db, &variant_id, &value_id, &payload.label, &payload.value).await;

    Ok(().into_response())
}


pub async fn delete_variant(
    db: DBExtension,
    _: OnlyInDev,
    Path(variant_id): Path<ObjectId>,
) -> HandlerResult {
    // TODO: omer
    // 1. check if any category uses it
    // 2. if so - return error

    Ok(().into_response())
}

pub async fn delete_variant_value(
    db: DBExtension,
    _: OnlyInDev,
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