use super::{extract_insert_document_error, InsertDocumentErrors};
use crate::{
    db::models::{ContactUsForm, ContactUsReason, DBModel},
    helpers::types::DBExtension,
};

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
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
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
