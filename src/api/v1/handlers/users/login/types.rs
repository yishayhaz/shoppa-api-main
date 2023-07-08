use crate::prelude::types::*;

#[derive(Deserialize, Serialize, Validate)]
pub struct UserLoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
}
