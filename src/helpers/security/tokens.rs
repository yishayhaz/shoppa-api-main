use crate::db::models::{DBModel, User};
use crate::helpers::env::EnvVars;
use crate::helpers::types::ResponseBuilder;
use axum::response::{IntoResponse, Response};
use chrono::Utc;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref JWT_LOGIN_KEY: HS256Key =
        HS256Key::from_bytes(&EnvVars::JWT_LOGIN_SECRET.get().as_bytes());
}


#[derive(Debug, Serialize, Deserialize)]
pub struct LoginTokenData {
    pub id: String,
    pub level: i32,
    pub created_at: i64,
}

pub fn generate_login_token(user: &User) -> Result<String, Response> {
    let data = LoginTokenData {
        id: user.id()?.to_string(),
        level: user.level,
        created_at: Utc::now().timestamp(),
    };

    let claims = Claims::with_custom_claims(data, Duration::from_days(90));

    match JWT_LOGIN_KEY.authenticate(claims) {
        Ok(token) => Ok(token),
        Err(_) => Err(ResponseBuilder::<u16>::error(
            None,
            Some(String::from("Faild to create login token")),
            Some(500),
        )
        .into_response()),
    }
}

pub fn verify_login_token(token: &str) -> Result<JWTClaims<LoginTokenData>, Response> {
    // TODO improve this
    match JWT_LOGIN_KEY.verify_token::<LoginTokenData>(&token, None){
        Ok(claims) => Ok(claims),
        Err(_) => Err(ResponseBuilder::<u16>::error(
            None,
            Some(String::from("Faild to verify login token")),
            Some(500),
        )
        .into_response()),
    }
}
