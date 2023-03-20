use crate::{
    db::models::{DBModel, User},
    helpers::{env::EnvVars, types::ResponseBuilder},
};
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

lazy_static! {
    static ref LOGIN_TOKEN_KEY: PasetoSymmetricKey<V4, Local> =
        PasetoSymmetricKey::from(Key::from(EnvVars::LOGIN_TOKEN_SECRET.get().as_bytes()));
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
            .build(&LOGIN_TOKEN_KEY)?;

        Ok(token)
    };

    match token_builder() {
        Ok(token) => Ok(token),
        Err(_) => Err(ResponseBuilder::<u16>::error(
            // TODO add error code here
            "",
            None,
            Some("Internal Server Error while generating login token"),
            Some(500),
        )
        .into_response()),
    }
}

pub fn verify_login_token(token: &str) -> Result<LoginTokenData, Response> {
    // TODO remove cookie when token is invalid
    let invalid_token_response = Err(ResponseBuilder::<u16>::error(
        // TODO add error code here
        "",
        None,
        Some("Invalid login token"),
        Some(401),
    )
    .into_response());

    let token_data = match PasetoParser::<V4, Local>::default().parse(token, &LOGIN_TOKEN_KEY) {
        Ok(token) => token,
        Err(_) => return invalid_token_response,
    };

    let data = token_data.get("payload");

    let data = match data {
        Some(v) => v,
        None => return invalid_token_response,
    };

    let data = serde_json::from_value::<LoginTokenData>(data.clone());

    match data {
        Ok(v) => Ok(v),
        Err(_) => invalid_token_response,
    }
}
