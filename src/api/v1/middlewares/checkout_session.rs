use crate::{helpers::cookies::CookieManager, prelude::*, tokens::CHECKOUT_SESSION_TOKEN_MANAGER};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shoppa_core::ResponseBuilder;
use tower_cookies::Cookies;

#[derive(Debug, Clone)]
pub struct CurrentCheckOutSession {
    pub secret: String,
}

pub async fn checkout_session_required<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> StdResult<Response, Response> {
    let cookies = req.extensions().get::<Cookies>().ok_or(
        ResponseBuilder::error("", Some(()), Some("FAILD TO GET COOKIES"), Some(500))
            .into_response(),
    )?;

    let session_cookie = &cookies
        .get_checkout_session_cookie()
        .ok_or(ResponseBuilder::error("", Some(()), None, Some(401)).into_response())?;

    if let Ok(data) = CHECKOUT_SESSION_TOKEN_MANAGER.decode_token(session_cookie) {
        req.extensions_mut()
            .insert(CurrentCheckOutSession::new(data.secret));

        Ok(next.run(req).await)
    } else {
        cookies.delete_checkout_session_cookie();
        Err(ResponseBuilder::error("", Some(()), None, Some(403)).into_response())
    }
}

impl CurrentCheckOutSession {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for CurrentCheckOutSession
where
    S: Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> StdResult<Self, Self::Rejection> {
        parts
            .extensions
            .remove::<CurrentCheckOutSession>()
            .ok_or(ResponseBuilder::error("", Some(()), None, Some(500)).into_response())
    }
}
