use super::types;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, ResponseBuilder};

pub async fn create_store_user(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStoreUserPayload>,
) -> HandlerResult {
    let store = db.get_store_by_id(&payload.store, None, None, None).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<()>::error("Store not found", None, None, Some(404)).into_response(),
        );
    }

    let store_user = db.insert_new_store_user(payload, None, None).await?;

    // TODO send email to user and generate registration link

    Ok(ResponseBuilder::success(Some(store_user), None, None).into_response())
}
