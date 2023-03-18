use super::InsertDocumentErrors;
use crate::{
    db::models::{ContactUsForm, ContactUsReason, DBModel},
    helpers::types::DBExtension,
};
use mongodb::error::ErrorKind;

type InsertContactUsFormResult = Result<ContactUsForm, InsertDocumentErrors>;

pub async fn new_contact_us_request(
    db: &DBExtension,
    email: String,
    message: String,
    reason: ContactUsReason,
) -> InsertContactUsFormResult {
    let mut contact_us = ContactUsForm::new(email, message, reason);

    let res = match db.contact_us_form.insert_one(&contact_us, None).await {
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

    contact_us.update_id(id);

    Ok(contact_us)
}
