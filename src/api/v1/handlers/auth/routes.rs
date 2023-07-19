use super::types::LoginPayload;
use crate::api::v1::middlewares::CurrentUser;
use crate::db::UserFunctions;
use crate::helpers::cookies::CookieManager;
use crate::{db::AxumDBExtansion, prelude::*};
use axum::{
    extract::{Json, Query, Extension},
    response::IntoResponse,
};
use shoppa_core::ResponseBuilder;
use shoppa_core::{
    db::models::{EmbeddedDocument, ProductItemStatus, ProductStatus},
    extractors::JsonWithValidation,
};
use tower_cookies::Cookies;

pub async fn login(
    db: AxumDBExtansion,
    cookies: Cookies,
    Extension(mut current_user): Extension<Option<CurrentUser>>,
    JsonWithValidation(payload): JsonWithValidation<LoginPayload>,
) -> HandlerResult {
   

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

pub async fn logout(
    cookies: Cookies,
    // current_user: CurrentUser,
) -> HandlerResult {
   
    cookies.delete_access_cookie();

    Ok(ResponseBuilder::<()>::success(None, None, None).into_response())
}

