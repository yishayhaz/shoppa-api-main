use crate::{
    helpers::cookies::CookieManager, helpers::types::Cookeys, tokens::STORE_USER_TOKEN_MANAGER,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use bson::oid::ObjectId;
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;

// Use this struct to get the current user data in the request handler
// This will work only in the context of the login_required middleware
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub user_id: ObjectId,
    pub token_secret: String,
}

pub async fn login_required<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Response> {
    let cookies = req.extensions().get::<Cookies>().ok_or(
        ResponseBuilder::error("", Some(()), Some("FAILD TO GET COOKIES"), Some(500))
            .into_response(),
    )?;

    let access_cookie = &cookies
        .get(Cookeys::StoreUserAccessToken.to_string().as_str())
        .ok_or(ResponseBuilder::error("", Some(()), None, Some(401)).into_response())?;

    let token_data = STORE_USER_TOKEN_MANAGER.decode_token(access_cookie.value());

    if let Ok(data) = token_data {
        req.extensions_mut().insert(CurrentUser {
            user_id: data.user_id,
            token_secret: data.token_secret,
        });

        Ok(next.run(req).await)
    } else {
        cookies.delete_cookie(&Cookeys::StoreUserAccessToken);
        Err(ResponseBuilder::error("", Some(()), None, Some(403)).into_response())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentUser
where
    S: Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .remove::<CurrentUser>()
            .ok_or(ResponseBuilder::error("", Some(()), None, Some(500)).into_response())
    }
}
