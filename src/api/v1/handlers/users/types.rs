use super::super::prelude::types::*;


#[derive(Deserialize, Serialize, Validate)]
pub struct UserLoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserRegisterPayload {
    #[validate(email)]
    pub email: String,
    #[validate(custom = "password_validator")]
    pub password: String,
    #[validate(custom = "username_validator")]
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserUpdatePayload {
    pub email: String,
    pub phone_number: String,
    pub username: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct UserUpdatePasswordPayload {
    #[validate(custom = "password_validator")]
    pub old_password: String,
    #[validate(custom = "password_validator")]
    pub new_password: String,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ChangePasswordPayload {
    #[validate(custom = "password_validator")]
    pub old_password: String,
    #[validate(custom = "password_validator")]
    pub new_password: String,
}

impl Validate for UserUpdatePayload {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.email.is_empty() && self.phone_number.is_empty() && self.username.is_empty() {
            errors.add(
                "email",
                ValidationError::new("At least one of the fields is required"),
            );
            errors.add(
                "phone_number",
                ValidationError::new("At least one of the fields is required"),
            );
            errors.add(
                "name",
                ValidationError::new("At least one of the fields is required"),
            );
        };

        if !self.email.is_empty() {
            if !validator::validate_email(&self.email) {
                errors.add("email", ValidationError::new("Invalid email"));
            }
        }

        if !self.phone_number.is_empty() {
            // I want to get the predefined error message from the phone_number_validator function
            let valid = phone_number_validator(&self.phone_number);
            if valid.is_err() {
                errors.add("phone_number", valid.err().unwrap());
            }
        }

        if !self.username.is_empty() {
            // I want to get the predefined error message from the username_validator function
            let valid = username_validator(&self.username);
            if valid.is_err() {
                errors.add("name", valid.err().unwrap());
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
