use super::types::CreateVariantPayload;
use crate::{
    db::{inserts},
    helpers::{
        types::{DBExtension, HandlerResponse, ResponseBuilder},
        json::JsonWithValidation
    },
};
use axum::{response::IntoResponse, Json};
use tower_cookies::Cookies;

pub async fn create_new_variant(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<CreateVariantPayload>,
) -> HandlerResponse {

    let _ = inserts::new_variant(&db, payload.name, payload.values).await;

    Ok(().into_response())
}