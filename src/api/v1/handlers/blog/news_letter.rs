use super::types;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, ResponseBuilder};

pub async fn signup_to_news_letter(
    db: AxumDBExtansion,
    JsonWithValidation(payload): JsonWithValidation<types::SignUpToNewsLetterPayload>,
) -> HandlerResult {
    db.insert_new_news_letter_subscriber(payload, None, None).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
