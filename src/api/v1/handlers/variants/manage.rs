use super::types::CreateVariantPayload;
use crate::{
    api::v1::middlewares::*,
    db::inserts,
    prelude::{handlers::*, *},
};

pub async fn create_new_variant(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<CreateVariantPayload>,
) -> HandlerResult {
    let _ = inserts::new_variant(&db, payload.name, payload.values, payload.type_).await;

    Ok(().into_response())
}
