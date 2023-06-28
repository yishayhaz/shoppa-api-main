use crate::prelude::{types::*, *};
use shoppa_core::{
    validators,
};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct CompleteRegistrationPayload {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(custom = "validators::password_validator")]
    pub password: String,
    #[validate(custom = "validators::username_validator")]
    pub name: Option<String>
}
