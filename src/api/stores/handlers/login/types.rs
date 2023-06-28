use crate::prelude::types::*;
use shoppa_core::validators;
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "validators::password_validator")]
    pub password: String,
}
