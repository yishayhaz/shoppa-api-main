use crate::{
    api::v1::middlewares::*,
    db::AxumDBExtansion,
    helpers::{cookies::delete_cookie, types::Cookeys},
    prelude::*,
};
use tower_cookies::Cookies;
use shoppa_core::ResponseBuilder;
use axum::response::IntoResponse;

pub async fn get_me(
    db: AxumDBExtansion,
    cookies: Cookies,
    GetTokenForGetMe(token_data): GetTokenForGetMe,
) -> HandlerResult {
    let user = db.get_user_by_id(&token_data.user_id, None, None, None).await?;

    match user {
        Some(user) => {
            Ok(ResponseBuilder::success(Some(user.to_get_me()?), None, None).into_response())
        }
        // TODO add error code
        None => {
            cookies.remove(delete_cookie(&Cookeys::AccessToken));

            Ok(ResponseBuilder::<u16>::error("", None, None, None).into_response())
        }
    }
}
