use super::types;
use crate::{
    db::{inserts, inserts::InsertDocumentErrors},
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
    let _ = match inserts::new_news_letter_subscriber(&db, payload.email).await {
        Ok(_) => {}
        Err(e) => match e {
            InsertDocumentErrors::UnknownError => {
                return Err(ResponseBuilder::<u16>::error("", None, None, None).into_response());
            }
            InsertDocumentErrors::AlredyExists => {
                return Err(ResponseBuilder::<u16>::error(
                    "",
                    None,
                    Some("looks like you alredy subscribed"),
                    Some(409),
                )
                .into_response());
            }
        },
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
