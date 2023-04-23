use super::{super::prelude::routes::*, types};
use crate::db::inserts;
use crate::db::models::ContactUsForm;
use crate::db::queries;

pub async fn contact_us_request(
  db: DBExtension,
  JsonWithValidation(payload): JsonWithValidation<types::ContactUsPayload>,
) -> HandlerResponse {
  let _ =
      inserts::new_contact_us_request(&db, payload.email, payload.message, payload.reason).await;

  Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub async fn get_contact_us(
  db: DBExtension,
  _: OnlyInDev,
  Query(query): Query<types::GetProductQueryParams>,
) -> HandlerResponse {
  let forms = queries::get_contact_us_forms(&db).await?;

  Ok(ResponseBuilder::<Vec<ContactUsForm>>::success(Some(forms), None, None).into_response())
}

pub async fn update_status(
  db: DBExtension,
  _: OnlyInDev,
  Path(form_oid): Path<ObjectId>,
)