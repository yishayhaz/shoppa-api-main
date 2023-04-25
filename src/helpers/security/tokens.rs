use crate::{
    db::models::{DBModel, User},
    helpers::env::ENV_VARS,
    prelude::*,
};
use bson::oid::ObjectId;
use chrono::Utc;
use rusty_paseto::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

lazy_static! {
    static ref LOGIN_TOKEN_KEY: PasetoSymmetricKey<V4, Local> =
        PasetoSymmetricKey::from(Key::from(ENV_VARS.LOGIN_TOKEN_SECRET.as_bytes()));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenData {
    pub user_id: ObjectId,
    pub level: i32,
}

pub fn generate_login_token(user: &User) -> Result<String> {
    let user_id = user.id()?.to_string();

    let token_builder = || -> StdResult<String, Box<dyn std::error::Error>> {
        let in_90_days = (Utc::now() + chrono::Duration::days(90)).to_rfc3339();

        let token = PasetoBuilder::<V4, Local>::default()
            .set_claim(ExpirationClaim::try_from(in_90_days)?)
            .set_claim(IssuerClaim::try_from("main-api")?)
            .set_claim(CustomClaim::try_from(("level", user.level))?)
            .set_claim(CustomClaim::try_from(("user_id", user_id))?)
            .build(&LOGIN_TOKEN_KEY)?;
        Ok(token)
    };

    match token_builder() {
        Ok(token) => Ok(token),
        Err(_) => Err(Error::Static("TODO")),
    }
}

pub fn decode_login_token(token: &str) -> Result<LoginTokenData> {
    let token_data = match PasetoParser::<V4, Local>::default().parse(token, &LOGIN_TOKEN_KEY) {
        Ok(token) => token,
        Err(_) => return Err(Error::Static("TODO")),
    };

    let data = serde_json::from_value::<LoginTokenData>(json!({
        "level": token_data.get("level"),
        "user_id":  token_data.get("user_id")
    }));

    match data {
        Ok(v) => Ok(v),
        Err(_) => Err(Error::Static("TODO")),
    }
}
