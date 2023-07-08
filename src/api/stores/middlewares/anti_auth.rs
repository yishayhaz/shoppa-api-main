use crate::{
    helpers::cookies::CookieManager, helpers::types::Cookeys, tokens::STORE_USER_TOKEN_MANAGER,
};
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;

pub async fn guest_required<B>(req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let cookies = req.extensions().get::<Cookies>().ok_or(
        ResponseBuilder::error("", Some(()), Some("FAILD TO GET COOKIES"), Some(500))
            .into_response(),
    )?;

    let access_cookie = &cookies.get(Cookeys::StoreUserAccessToken.to_string().as_str());

    if let Some(access_cookie) = access_cookie {
        let token_data = STORE_USER_TOKEN_MANAGER.decode_token(access_cookie.value());

        if let Ok(_) = token_data {
            Err(
                ResponseBuilder::error("", Some(()), Some("Need to be guest"), Some(401))
                    .into_response(),
            )
        } else {
            cookies.delete_cookie(&Cookeys::StoreUserAccessToken);
            Ok(next.run(req).await)
        }
    } else {
        return Ok(next.run(req).await);
    }
}
