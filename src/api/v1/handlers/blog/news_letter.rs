use super::types;
use crate::{
    db::inserts,
    helpers::{
        json::JsonWithValidation,
        types::{DBExtension, HandlerResponse, ResponseBuilder},
    },
};
use axum::response::IntoResponse;

pub async fn signup_to_news_letter(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::SignUpToNewsLetterPayload>,
) -> HandlerResponse {
    let _ = inserts::new_news_letter_subscriber(&db, payload.email).await;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
