use super::types;
use crate::{api::v1::middlewares::*, db::AxumDBExtansion, prelude::*};
use axum::{
    extract::{Json, Path, Query},
    response::IntoResponse,
};
use shoppa_core::{extractors::JsonWithValidation, ResponseBuilder};

pub async fn contact_us_request(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::ContactUsPayload>,
) -> HandlerResult {
    db.insert_new_contact_us_form(payload, None).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}