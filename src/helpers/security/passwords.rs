use crate::helpers::types::ResponseBuilder;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::response::{IntoResponse, Response};

pub fn hash_password(password: &str) -> Result<String, Response> {
    let argon2: Argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(v) => v,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                None,
                Some(String::from("Internal Server Error while hashing password")),
                Some(500),
            )
            .into_response())
        }
    };

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash_password: &str) -> Result<bool, Response> {

    let parsed_hash = match PasswordHash::new(&hash_password) {
        Ok(v) => v,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                None,
                Some(String::from(
                    "Internal Server Error while parsing password hash",
                )),
                Some(500),
            )
            .into_response())
        }
    };
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    Ok(parsed_hash.verify_password(algs, &password).is_ok())
}
