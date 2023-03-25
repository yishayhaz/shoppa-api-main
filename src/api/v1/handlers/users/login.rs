use super::super::prelude::routes::*;
use super::types::UserLoginPayload;
use crate::{
    db::queries,
    helpers::{
        cookies::{delete_cookie, set_access_cookie},
        security,
        types::Cookeys,
    },
};

pub async fn login(
    db: DBExtension,
    cookies: Cookies,
    JsonWithValidation(payload): JsonWithValidation<UserLoginPayload>,
) -> HandlerResponse {
    let user = queries::get_user_by_email(&db, payload.email).await?;

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
        None => return Err(user_not_found),
    };

    if !security::verify_password(&payload.password, user_password)? {
        return Err(user_not_found);
    }

    set_access_cookie(&cookies, &user)?;

    Ok(ResponseBuilder::success(Some(user), None, None).into_response())
}

pub async fn logout(cookies: Cookies) -> HandlerResponse {
    cookies.add(delete_cookie(&Cookeys::AccessToken));

    Ok(ResponseBuilder::<u16>::success(None, None, None).into_response())
}
