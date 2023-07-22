use super::types::{LoginPayload, SignupPayload};
use crate::api::v1::middlewares::CurrentUser;
use crate::{
    db::{AxumDBExtansion, UserFunctions},
    helpers::cookies::CookieManager,
    prelude::*,
};
use axum::{extract::Extension, response::IntoResponse};
use bson::doc;
use shoppa_core::{
    constans,
    db::models::{DBModel, User},
    extractors::JsonWithValidation,
    security, ResponseBuilder,
};
use tower_cookies::Cookies;

pub async fn login(
    db: AxumDBExtansion,
    cookies: Cookies,
    Extension(current_user): Extension<Option<CurrentUser>>,
    JsonWithValidation(payload): JsonWithValidation<LoginPayload>,
) -> HandlerResult {
    let user = db.get_user_by_email(&payload.email, None, None).await?;

    let not_found_response =
        ResponseBuilder::<()>::error("UserNotFound", None, None, Some(404)).into_response();

    if user.is_none() {
        // we pretend that the user exists to avoid timing attacks
        security::verify_password(&payload.password, constans::INVALID_PASSWORD_VALID_HASH)?;
        return Ok(not_found_response);
    }

    let mut user = user.unwrap();

    let password = match user.password {
        Some(ref password) => password.as_str(),
        None => constans::INVALID_PASSWORD_VALID_HASH,
    };

    if !security::verify_password(&payload.password, password)? {
        return Ok(not_found_response);
    }

    if let Some(mut current_user) = current_user {
        current_user.fetch(&db, None).await?;
        if current_user.user_exists() {
            user.cart = current_user.user().unwrap().cart + user.cart;

            // If there was error updating the user cart we just ignore it
            let _ = db.update_user_by_id(
                user.id().unwrap(),
                doc! {
                    "$set": {
                        User::fields().cart: &user.cart
                    }
                },
                None,
                None,
            )
            .await;
        }
    };

    cookies.set_access_cookie(&user)?;
    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn logout(
    cookies: Cookies,
    // current_user: CurrentUser,
) -> HandlerResult {
    cookies.delete_access_cookie();

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn signup(
    db: AxumDBExtansion,
    cookies: Cookies,
    Extension(current_user): Extension<Option<CurrentUser>>,
    JsonWithValidation(payload): JsonWithValidation<SignupPayload>,
) -> HandlerResult {
    let mut user: User = payload.try_into()?;

    // If there is a current guest user we set the cart to the new user
    if let Some(mut current_user) = current_user {
        current_user.fetch(&db, None).await?;
        if let Some(c_user) = current_user.user() {
            user.cart = c_user.cart;
        }
    };

    let user = db.insert_new_user(user, None, None).await?;

    cookies.set_access_cookie(&user)?;

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}
