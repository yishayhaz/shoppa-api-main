use crate::{
    helpers::{cookies::CookieManager, types::Cookeys},
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;

pub async fn logout(cookies: Cookies) -> HandlerResult {
    cookies.delete_cookie(&Cookeys::StoreUserAccessToken);

    Ok(ResponseBuilder::success(Some(""), None, None).into_response())
}
