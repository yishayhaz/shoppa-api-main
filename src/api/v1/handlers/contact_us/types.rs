use crate::{
    db::models::{ContactFormStatus, ContactUsReason},
    prelude::types::*,
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
