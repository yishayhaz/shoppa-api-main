use super::types::UserLoginPayload;
use crate::{
    api::v1::middlewares::*,
    helpers::{
        cookies::{delete_cookie, set_access_cookie},
        types::Cookeys,
    },
};
use crate::{
    db::{AxumDBExtansion, UserFunctions},
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::{extractors::JsonWithValidation, security, ResponseBuilder};
use tower_cookies::Cookies;

pub async fn login(
    db: AxumDBExtansion,
    cookies: Cookies,
    GuestOnly(_): GuestOnly,
    JsonWithValidation(payload): JsonWithValidation<UserLoginPayload>,
) -> HandlerResult {
    let user = db.get_user_by_email(&payload.email, None, None).await?;

    let user_not_found =
    //TODO add error code here
        ResponseBuilder::<u16>::error("", None, Some("User not found"), Some(404))
            .into_response();

    let user = match user {
        Some(user) => user,
        None => return Ok(user_not_found),
    };

    let user_password = match &user.password {
        Some(user_password) => user_password,
        None => return Ok(user_not_found),
    };

    if !security::verify_password(&payload.password, user_password)? {
        return Ok(user_not_found);
    }

    set_access_cookie(&cookies, &user)?;

    Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
}

pub async fn logout(cookies: Cookies, Level2Access(_): Level2Access) -> HandlerResult {
    cookies.add(delete_cookie(&Cookeys::AccessToken));

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
