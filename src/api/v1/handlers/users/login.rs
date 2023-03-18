use super::types::UserLoginPayload;
use crate::{
    db::queries,
    helpers::{
        cookies::{create_cookie, delete_cookie},
        json::JsonWithValidation,
        security,
        types::{Cookeys, DBExtension, HandlerResponse, ResponseBuilder, MAX_COOKIE_EXP},
    },
};
use axum::response::IntoResponse;
use tower_cookies::Cookies;

pub async fn login(
    db: DBExtension,
    cookies: Cookies,
    JsonWithValidation(payload): JsonWithValidation<UserLoginPayload>,
) -> HandlerResponse {
    let user = queries::get_user_by_email(&db, payload.email).await?;

    let user_not_found =
        ResponseBuilder::<u16>::error(None, Some(String::from("User not found")), Some(404))
            .into_response();

    let user = match user {
        Some(user) => user,
        None => return Ok(user_not_found),
    };

    let user_password = match &user.password {
        Some(user_password) => user_password,
        None => return Err(user_not_found),
    };

    if !security::verify_password(&payload.password, user_password)? {
        return Err(user_not_found);
    }

    let login_token = security::generate_login_token(&user)?;

    let login_token_cookie =
        create_cookie(&Cookeys::AccessToken, login_token, MAX_COOKIE_EXP, true);

    cookies.add(login_token_cookie);

    Ok(ResponseBuilder::success(Some(user), None, None).into_response())
}

pub async fn logout(cookies: Cookies) -> HandlerResponse {
    cookies.add(delete_cookie(&Cookeys::AccessToken));

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
