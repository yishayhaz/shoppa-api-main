use super::types;
use crate::{
    db::{inserts, queries, updates},
    prelude::{handlers::*, *},
    api::v1::middlewares::*,
};

pub async fn contact_us_request(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::ContactUsPayload>,
) -> HandlerResult {
    let _ =
        inserts::new_contact_us_request(&db, payload.email, payload.message, payload.reason).await;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn get_contact_us(
    db: DBExtension,
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
    db: DBExtension,
    _: OnlyInDev,
    Path(form_id): Path<ObjectId>,
    Json(payload): Json<types::UpdateContactUsPayload>,
) -> HandlerResult {
    let contact_us = updates::update_contact_us_by_id(&db, form_id, payload.status).await?;

    Ok(ResponseBuilder::success(contact_us, None, None).into_response())
}
