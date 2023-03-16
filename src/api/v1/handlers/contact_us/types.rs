use serde::{Deserialize, Serialize};
use crate::db::models;
use validator::{Validate};


#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ContactUsPayload{
    pub reason: models::ContactUsReason,
    #[validate(email)]
    pub email: String,
    pub message: String
}