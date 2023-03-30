use super::{super::prelude::routes::*, types};
use crate::db::inserts;

pub async fn create_new_store(
    db: DBExtension,
    _: OnlyInDev,
    JsonWithValidation(payload): JsonWithValidation<types::CreateStorePayload>,
) -> HandlerResponse {

    let _ = inserts::new_store(&db, payload.name, payload.email, payload.location).await;

    Ok(ResponseBuilder::success(Some("dad"), None, None).into_response())
}
