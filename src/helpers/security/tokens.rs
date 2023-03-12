use crate::db::models::{DBModel, User};
use crate::helpers::env::EnvVars;
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

lazy_static! {
    static ref JWT_LOGIN_KEY: PasetoSymmetricKey<V4, Local> =
        PasetoSymmetricKey::from(Key::from(EnvVars::JWT_LOGIN_SECRET.get().as_bytes()));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenData {
    pub id: String,
    pub level: i32,
}

pub fn generate_login_token(user: &User) -> Result<String, Response> {
    let token_data = LoginTokenData {
        id: user.id()?.to_string(),
        level: user.level,
    };

    let token_builder = || -> Result<String, Box<dyn Error>> {
        let in_90_days = (Utc::now() + chrono::Duration::days(90)).to_rfc3339();

        let token = PasetoBuilder::<V4, Local>::default()
            .set_claim(ExpirationClaim::try_from(in_90_days)?)
            .set_claim(IssuerClaim::try_from("main-api")?)
            .set_claim(CustomClaim::try_from(("payload", token_data))?)
            .build(&JWT_LOGIN_KEY)?;

        Ok(token)
    };

    match token_builder() {
        Ok(token) => Ok(token),
        Err(_) => Err(ResponseBuilder::<u16>::error(
            None,
            Some(String::from(
                "Internal Server Error while generating login token",
            )),
            Some(500),
        )
        .into_response()),
    }
}
