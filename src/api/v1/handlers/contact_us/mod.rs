mod types;
use super::prelude::routes::*;
use crate::db::inserts;
use axum::{routing, Router};

async fn contact_us_request(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::ContactUsPayload>,
) -> HandlerResponse {
    let _ =
        inserts::new_contact_us_request(&db, payload.email, payload.message, payload.reason).await;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}

pub fn router() -> Router {
    Router::new().route("/", routing::post(contact_us_request))
}
