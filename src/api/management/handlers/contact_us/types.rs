use crate::prelude::{types::*};
use shoppa_core::{
    db::models::{ContactFormStatus, ContactUsReason, ContactUsForm},
    parser::empty_string_as_none,
};

#[derive(Debug, Validate, Deserialize)]
pub struct ContactUsPayload {
    pub reason: ContactUsReason,
    #[validate(email)]
    pub email: String,
    pub message: String,
}
#[derive(Debug, Deserialize)]
pub struct GetContactUsQueryParams {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    pub status: Option<ContactFormStatus>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct UpdateContactUsPayload {
    pub status: ContactFormStatus,
}


impl Into<ContactUsForm> for ContactUsPayload {
    fn into(self) -> ContactUsForm {
        ContactUsForm::new(self.email, self.message, self.reason)
    }
}
