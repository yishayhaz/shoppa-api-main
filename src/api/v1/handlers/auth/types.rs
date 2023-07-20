use crate::prelude::types::*;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct SignupPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
    #[validate(custom = "username_validator")]
    pub name: String,
    #[validate(custom = "phone_number_validator")]
    pub phone_number: Option<String>,
    pub date_of_birth: Option<DateTime<Utc>>
}