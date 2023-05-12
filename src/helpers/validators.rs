use super::types::error_code;
use std::borrow::Cow;
use validator::ValidationError;

const MAX_USERNAME_LENGTH: usize = 64;
// Two words, 2 letters each, 1 space in between
const MIN_USERNAME_LENGTH: usize = 5;

pub fn valid_phone_number(phone_number: &str) -> bool {
    if phone_number.len() != 12 {
        return false;
    }
    // TODO in v2 add support for other countries
    if !phone_number.starts_with("+9725") {
        return false;
    }

    if !phone_number.chars().all(|c| c.is_digit(10)) {
        return false;
    }

    true
}

pub fn valid_password(password: &str) -> bool {
    let password_length = password.len();

    if password_length < 8 || password_length > 64 {
        return false;
    }

    if !password
        .chars()
        .any(|c| c.is_lowercase() || c.is_uppercase())
    {
        return false;
    }

    if !password.chars().any(|c| c.is_digit(10)) {
        return false;
    }

    true
}

pub fn valid_username(username: &str) -> bool {
    let username_length = username.len();

    if username_length < MIN_USERNAME_LENGTH || username_length > MAX_USERNAME_LENGTH {
        return false;
    }

    let words: Vec<&str> = username.split_whitespace().collect();

    if words.len() < 2 {
        return false;
    }

    for word in words {
        if word.len() < 2 {
            return false;
        } else if !word.chars().all(|c| c.is_alphabetic()) {
            return false;
        }
    }

    true
}

pub fn phone_number_validator(phone_number: &str) -> Result<(), ValidationError> {
    if !valid_phone_number(phone_number) {
        let mut error = ValidationError::new(error_code::INVALID_PHONE_NUMBER);

        error.message = Some(Cow::from("Invalid phone number"));

        return Err(error);
    }
    Ok(())
}

pub fn password_validator(password: &str) -> Result<(), ValidationError> {
    if !valid_password(password) {
        let mut error = ValidationError::new(error_code::INVALID_PASSWORD);

        error.message = Some(Cow::from("Invalid password, must be between 8 and 64 characters long, contain at least one letter and one number"));

        return Err(error);
    }
    Ok(())
}

pub fn username_validator(username: &str) -> Result<(), ValidationError> {
    if !valid_username(username) {
        let mut error = ValidationError::new(error_code::INVALID_USERNAME);

        error.message = Some(Cow::from("Invalid username, must be between 5 and 64 characters long, contain at least two words, each word must be at least 2 characters long and contain only letters"));

        return Err(error);
    }
    Ok(())
}
