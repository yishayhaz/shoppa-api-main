use crate::{
    api::v1::middlewares::*,
    db::AxumDBExtansion,
    helpers::{cookies::CookieManager, types::Cookeys},
    prelude::*,
};
use axum::response::IntoResponse;
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;

pub async fn get_me(
    db: AxumDBExtansion,
    cookies: Cookies,
    GetTokenForGetMe(token_data): GetTokenForGetMe,
) -> HandlerResult {
    let user = db
        .get_user_by_id(&token_data.user_id, None, None, None)
        .await?;

    match user {
        Some(user) => {
            Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
        }
        
        None => {
            cookies.delete_cookie(&Cookeys::AccessToken);

            Ok(ResponseBuilder::<u16>::error("", None, None, None).into_response())
        }
    }
}
