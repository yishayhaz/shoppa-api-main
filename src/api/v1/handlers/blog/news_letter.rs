use super::types;
use crate::{
    db::inserts,
    prelude::{handlers::*, *},
};

pub async fn signup_to_news_letter(
    db: DBExtension,
    JsonWithValidation(payload): JsonWithValidation<types::SignUpToNewsLetterPayload>,
) -> HandlerResult {
    let _ = inserts::new_news_letter_subscriber(&db, payload).await?;

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
