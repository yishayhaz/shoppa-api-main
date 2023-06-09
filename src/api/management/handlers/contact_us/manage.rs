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

pub async fn get_contact_us(
    db: AxumDBExtansion,
    pagination: Pagination,
    sorting: OptionalSorting<String>,
    _: OnlyInDev,
    Query(query): Query<types::GetContactUsQueryParams>,
) -> HandlerResult {
    let forms =
        queries::get_contact_us_forms(&db, Some(pagination), sorting.into(), query.status).await?;

    Ok(ResponseBuilder::success(Some(forms), None, None).into_response())
}

pub async fn update_status(
    db: AxumDBExtansion,
    _: OnlyInDev,
    Path(form_id): Path<ObjectId>,
    Json(payload): Json<types::UpdateContactUsPayload>,
) -> HandlerResult {
    let contact_us = updates::update_contact_us_by_id(&db, form_id, payload.status).await?;

    Ok(ResponseBuilder::success(contact_us, None, None).into_response())
}
