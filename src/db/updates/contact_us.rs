use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
};
use axum::response::IntoResponse;
use axum::response::Response;
use bson::{doc, oid::ObjectId, Document, Bson};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateContactUsResult = Result<Option<models::ContactUsForm>, Response>;

async fn _update_contact_us_form(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateContactUsResult {
    let contact_us = db.contact_us_form.find_one_and_update(filter, update, option).await.map_err(|e|{
        ResponseBuilder::query_error("contact_us", e).into_response()
    })?;

    Ok(contact_us)
}


pub async fn update_contact_us_by_id(
    db: &DBExtension,
    id: ObjectId,
    status: models::ContactFormStatus
) -> UpdateContactUsResult {

    let filter = doc! {
        "_id": id
    };
    
    let update = doc! {
        "$set": {
            "status": Into::<Bson>::into(status)
        }
    };

    _update_contact_us_form(
        db,
        filter,
        update,
        None
    ).await
}
