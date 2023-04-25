use super::types;
use crate::{
    db::{inserts, inserts::InsertDocumentErrors},
    prelude::{handlers::*, *},
};

pub async fn signup_to_news_letter(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::SignUpToNewsLetterPayload>,
) -> HandlerResult {
    let _ = match inserts::new_news_letter_subscriber(&db, payload.email).await {
        Ok(_) => {}
        Err(e) => match e {
            InsertDocumentErrors::AlredyExists => {
                return Ok(ResponseBuilder::<u16>::error(
                    "",
                    None,
                    Some("looks like you alredy subscribed"),
                    Some(409),
                )
                .into_response());
            }
            _ => return Ok(e.into_response()),
        },
    };

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
