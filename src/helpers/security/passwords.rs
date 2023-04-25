use crate::prelude::*;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub fn hash_password(password: &str) -> Result<String> {
    let argon2: Argon2 = Argon2::default();

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| Error::HashError(e))?;

    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash_password: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hash_password).map_err(|e| Error::HashError(e))?;
    let algs: &[&dyn PasswordVerifier] = &[&Argon2::default()];

    Ok(parsed_hash.verify_password(algs, &password).is_ok())
}
