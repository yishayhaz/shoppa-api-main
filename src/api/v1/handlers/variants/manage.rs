use super::super::prelude::routes::*;
use super::types::CreateVariantPayload;
use crate::db::inserts;

pub async fn create_new_variant(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<CreateVariantPayload>,
) -> HandlerResponse {
    let _ = inserts::new_variant(&db, payload.name, payload.values).await;

    Ok(().into_response())
}