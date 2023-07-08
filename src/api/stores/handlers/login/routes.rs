use super::types::LoginPayload;
use crate::{
    db::{AxumDBExtansion, StoreUserFunctions},
    helpers::{cookies::CookieManager, types::Cookeys},
    prelude::*,
    tokens::STORE_USER_TOKEN_MANAGER,
};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, security, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn login(
    db: AxumDBExtansion,
    cookies: Cookies,
    JsonWithValidation(payload): JsonWithValidation<LoginPayload>,
) -> HandlerResult {
    let user = db
        .get_store_user_by_email(payload.email.as_str(), true)
        .await?;

    let user_not_found =
        ResponseBuilder::<()>::error("user not found", None, Some("user not found"), Some(404))
            .into_response();

    // TODO in the future add delay to prevent guessing whether email exists or not
    if user.is_none() {
        return Ok(user_not_found);
    }

    let user = user.unwrap();

    if !security::verify_password(payload.password.as_str(), user.password.as_str())
        .unwrap_or(false)
    {
        return Ok(user_not_found);
    }

    let access_token = STORE_USER_TOKEN_MANAGER.generate_token(&user, None)?;

    cookies.set_cookie(
        &Cookeys::StoreUserAccessToken,
        access_token,
        90 * 24 * 60 * 60,
        true,
    );

    Ok(ResponseBuilder::success(Some(()), Some("login success"), Some(200)).into_response())
}

pub async fn _validate_2fa() -> HandlerResult {
    todo!()
}

pub async fn _login_with_google() -> HandlerResult {
    todo!()
}
