use super::types::{CompleteRegistrationPayload, ValidateRegistrationTokenPayload};
use crate::{
    db::{AxumDBExtansion, StoreUserFunctions},
    helpers::{cookies::CookieManager, types::Cookeys},
    prelude::*,
    tokens::{STORE_USER_REGISTRATION_TOKEN_MANAGER, STORE_USER_TOKEN_MANAGER},
};
use axum::response::IntoResponse;
use serde_json::json;
use shoppa_core::{extractors::JsonWithValidation, security, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn complete_registration(
    db: AxumDBExtansion,
    cookies: Cookies,
    JsonWithValidation(payload): JsonWithValidation<CompleteRegistrationPayload>,
) -> HandlerResult {
    let token_data = STORE_USER_REGISTRATION_TOKEN_MANAGER.decode_token(payload.token.as_str())?;

    let password = security::hash_password(payload.password.as_str())?;

    let user = db
        .get_store_user_by_id(&token_data.user_id, None, None, None)
        .await?;

    if user.is_none() {
        return Ok(ResponseBuilder::error("error_code", Some(()), None, Some(404)).into_response());
    }

    let user = user.unwrap();

    if user.registration_completed {
        return Ok(
            ResponseBuilder::error("reg completed alredy", Some(()), None, Some(400))
                .into_response(),
        );
    }

    if &user.registration_token_secret.unwrap_or_default() != &token_data.secret {
        return Ok(
            ResponseBuilder::error("new token had been issued", Some(()), None, Some(400))
                .into_response(),
        );
    }

    let user = db
        .complete_store_user_registration(
            &token_data.user_id,
            token_data.secret,
            password,
            payload.name,
        )
        .await?;

    if user.is_none() {
        return Ok(ResponseBuilder::error("error_code", Some(()), None, Some(404)).into_response());
    }

    let user = user.unwrap();

    let access_token = STORE_USER_TOKEN_MANAGER.generate_token(&user, None)?;

    cookies.set_cookie(
        &Cookeys::StoreUserAccessToken,
        access_token,
        90 * 24 * 60 * 60,
        true,
    );

    Ok(ResponseBuilder::success(
        Some(user),
        Some("registration completed successfully"),
        Some(201),
    )
    .into_response())
}

pub async fn validate_registration_token(
    JsonWithValidation(payload): JsonWithValidation<ValidateRegistrationTokenPayload>,
) -> HandlerResult {
    // TODO set a cookie that will be used to complete the registration
    let token_data = STORE_USER_REGISTRATION_TOKEN_MANAGER.decode_token(payload.token.as_str())?;

    Ok(ResponseBuilder::success(
        Some(json!(
            {
                "username": token_data.name,
            }
        )),
        None,
        None,
    )
    .into_response())
}
