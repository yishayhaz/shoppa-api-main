use super::InsertDocumentErrors;
use crate::{
    db::models::{DBModel, NewsLetterSubscriber},
    helpers::types::DBExtension,
};
use mongodb::error::ErrorKind;

type InsertContactUsFormResult = Result<NewsLetterSubscriber, InsertDocumentErrors>;

pub async fn new_news_letter_subscriber(
    db: &DBExtension,
    email: String,
) -> InsertContactUsFormResult {
    let mut news_letter_subscriber = NewsLetterSubscriber::new(email);

    let res = match db
        .news_letter_subscribers
        .insert_one(&news_letter_subscriber, None)
        .await
    {
        Ok(v) => v,
        Err(err) => match *err.kind {
            ErrorKind::Write(e) => {
                todo!("find a way to know if its a dup document");
                return Err(InsertDocumentErrors::UnknownError);
            }
            _ => {
                return Err(InsertDocumentErrors::UnknownError);
            }
        },
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    news_letter_subscriber.update_id(id);

    Ok(news_letter_subscriber)
}
