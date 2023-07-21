use crate::prelude::{types::*, *};
use chrono::{DateTime, Utc};
use shoppa_core::{
    db::models::{Genders, User},
    security,
};

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
    pub date_of_birth: Option<DateTime<Utc>>,
    pub gender: Option<Genders>,
}

impl TryInto<User> for SignupPayload {
    type Error = Error;
    fn try_into(self) -> Result<User> {
        Ok(User::new(
            self.name,
            self.email,
            self.phone_number,
            security::hash_password(&self.password)?,
            self.gender,
            self.date_of_birth,
        ))
    }
}
