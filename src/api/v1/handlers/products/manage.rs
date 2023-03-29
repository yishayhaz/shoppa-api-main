use super::super::prelude::routes::*;
use super::types::CreateProductPayload;
use crate::db::inserts;

pub async fn create_new_product(
    _db: DBExtension,
    _: OnlyInDev,
    Json(_payload): Json<CreateProductPayload>,
) -> HandlerResponse {
    Ok(().into_response())
}
