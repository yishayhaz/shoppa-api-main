use super::prelude::*;
use crate::{db::models::NewsLetterSubscriber, prelude::*};
use validator::Validate;

pub async fn new_news_letter_subscriber<T>(
    db: &DBExtension,
    news_letter_subscriber: T,
) -> Result<NewsLetterSubscriber>
where
    T: Into<NewsLetterSubscriber>,
{
    let mut news_letter_subscriber: NewsLetterSubscriber = news_letter_subscriber.into();

    news_letter_subscriber.validate()?;

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

pub async fn try_new_news_letter_subscriber<T>(
    db: &DBExtension,
    news_letter_subscriber: T,
) -> Result<NewsLetterSubscriber>
where
    T: TryInto<NewsLetterSubscriber>,
    T::Error: Into<Error>,
{
    let news_letter_subscriber = news_letter_subscriber.try_into().map_err(|e| e.into())?;

    new_news_letter_subscriber(db, news_letter_subscriber).await
}
