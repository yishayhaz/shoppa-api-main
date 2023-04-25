use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
    prelude::*,
};
use bson::{doc, oid::ObjectId, Bson, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateContactUsResult = Result<Option<models::ContactUsForm>>;

async fn _update_contact_us_form(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateContactUsResult {
    let contact_us = db
        .contact_us_form
        .find_one_and_update(filter, update, option)
        .await
        .map_err(|e| Error::DBError(("contact_us", e)))?;

    Ok(contact_us)
}

pub async fn update_contact_us_by_id(
    db: &DBExtension,
    id: ObjectId,
    status: models::ContactFormStatus,
) -> UpdateContactUsResult {
    let filter = doc! {
        "_id": id
    };

    let update = doc! {
        "$set": {
            "status": Into::<Bson>::into(status)
        }
    };

    _update_contact_us_form(db, filter, update, None).await
}
