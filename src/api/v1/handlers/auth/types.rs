use crate::prelude::types::*;

#[derive(Deserialize, Serialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
}