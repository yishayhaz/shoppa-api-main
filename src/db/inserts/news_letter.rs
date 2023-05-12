use super::prelude::*;
use crate::{db::models::NewsLetterSubscriber, prelude::*};

type InsertContactUsFormResult = Result<NewsLetterSubscriber>;

pub async fn new_news_letter_subscriber(
    db: &DBExtension,
    email: String,
) -> InsertContactUsFormResult {
    let mut news_letter_subscriber = NewsLetterSubscriber::new(email);

    let res = db
        .news_letter_subscribers
        .insert_one(&news_letter_subscriber, None)
        .await
        .map_err(|e| Error::DBError(("news_letter_subscribers", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    news_letter_subscriber.update_id(id);

    Ok(news_letter_subscriber)
}
