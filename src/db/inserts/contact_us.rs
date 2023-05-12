use super::prelude::*;
use crate::{
    db::models::{ContactUsForm, ContactUsReason},
    prelude::*,
};

type InsertContactUsFormResult = Result<ContactUsForm>;

pub async fn new_contact_us_request(
    db: &DBExtension,
    email: String,
    message: String,
    reason: ContactUsReason,
) -> InsertContactUsFormResult {
    let mut contact_us = ContactUsForm::new(email, message, reason);

    let res = db
        .contact_us_form
        .insert_one(&contact_us, None)
        .await
        .map_err(|e| Error::DBError(("contact_us_form", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    contact_us.update_id(id);

    Ok(contact_us)
}
