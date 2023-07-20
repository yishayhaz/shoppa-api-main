use super::types::{LoginPayload, SignupPayload};
use crate::api::v1::middlewares::CurrentUser;
use crate::{db::{AxumDBExtansion, UserFunctions}, prelude::*, helpers::cookies::CookieManager};
use axum::{
    extract::{Json, Query, Extension},
    response::IntoResponse,
};
use shoppa_core::{
    db::models::{EmbeddedDocument, ProductItemStatus, ProductStatus},
    extractors::JsonWithValidation,ResponseBuilder,
    security
};
use tower_cookies::Cookies;

pub async fn login(
    db: AxumDBExtansion,
    cookies: Cookies,
    Extension(mut current_user): Extension<Option<CurrentUser>>,
    JsonWithValidation(payload): JsonWithValidation<LoginPayload>,
) -> HandlerResult {
   
   let user = db
        .get_user_by_email(&payload.email, None, None)
        .await?;

    let not_found_response = ResponseBuilder::<()>::error("UserNotFound", None, None, Some(404)).into_response();

    if user.is_none() {
        // we pretend that the user exists to avoid timing attacks
        // TODO insert a valid password hash here
        security::verify_password(&payload.password, "")?;
        return Ok(not_found_response);
    }

    let user = user.unwrap();

    if !security::verify_password(&payload.password, &user.password.unwrap_or_default())? {
        return Ok(not_found_response);
    }

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
    Extension(mut current_user): Extension<Option<CurrentUser>>,
    JsonWithValidation(payload): JsonWithValidation<SignupPayload>,
) -> HandlerResult {
   
   tracing::info!("{:?}", payload.date_of_birth);

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}