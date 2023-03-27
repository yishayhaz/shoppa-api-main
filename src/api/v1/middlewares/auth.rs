use crate::helpers::{
    cookies::delete_cookie,
    security::{decode_login_token, LoginTokenData},
    types::{Cookeys, ResponseBuilder},
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use tower_cookies::Cookies;

pub struct GuestOnly(pub ());
// get me is a bit diffrent, we dont want to return an 400 when he is not logged in
// to help the seo
pub struct GetTokenForGetMe(pub LoginTokenData);
pub struct Level1AccessOrNone(pub Option<LoginTokenData>);
pub struct Level1Access(pub LoginTokenData);
pub struct Level2Access(pub LoginTokenData);
pub struct Level3Access(pub LoginTokenData);


pub enum AuthErrors {
    InvalidToken,
    MissingToken,
    FaildExtractingCookies,
    InsufficientLevel,
    GuestRequired,
    AuthErrorWith200
}

#[async_trait]
impl<S> FromRequestParts<S> for GuestOnly
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            Ok((_, _)) => Err(AuthErrors::GuestRequired),
            Err((_, _)) => Ok(GuestOnly(())),
        }
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for GetTokenForGetMe
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            // the min level is 1 so there is no need to check for the user level
            Ok((data, _)) => Ok(GetTokenForGetMe(data)),
            Err((e, _)) => Err(AuthErrors::AuthErrorWith200),
        }
    }
}


#[async_trait]
impl<S> FromRequestParts<S> for Level1AccessOrNone
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            Ok((data, _)) => {
                if data.level > 1 {
                    return Err(AuthErrors::GuestRequired);
                }

                Ok(Level1AccessOrNone(Some(data)))
            }
            Err((_, _)) => Ok(Level1AccessOrNone(None)),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Level1Access
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            // the min level is 1 so there is no need to check for the user level
            Ok((data, _)) => Ok(Level1Access(data)),
            Err((e, _)) => Err(e),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Level2Access
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            Ok((data, _)) => {
                if data.level < 2 {
                    return Err(AuthErrors::InsufficientLevel);
                }

                Ok(Level2Access(data))
            }
            Err((e, _)) => Err(e),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Level3Access
where
    S: Send + Sync,
{
    type Rejection = AuthErrors;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match extract_access_token(parts, state).await {
            // the min level is 1 so there is no need to check for the user level
            Ok((data, _)) => {
                if data.level < 3 {
                    return Err(AuthErrors::InsufficientLevel);
                }

                Ok(Level3Access(data))
            }
            Err((e, _)) => Err(e),
        }
    }
}

async fn extract_access_token<S>(
    parts: &mut Parts,
    _state: &S,
) -> Result<(LoginTokenData, Cookies), (AuthErrors, Option<Cookies>)>
where
    S: Send + Sync,
{
    let cookies = match parts.extensions.get::<Cookies>().cloned() {
        Some(c) => c,
        None => {
            return Err((AuthErrors::FaildExtractingCookies, None));
        }
    };

    let login_cookie = match cookies.get(Cookeys::AccessToken.get()) {
        Some(c) => c,
        None => {
            return Err((AuthErrors::MissingToken, Some(cookies)));
        }
    };

    let token_data = match decode_login_token(login_cookie.value()) {
        Ok(d) => d,
        Err(_) => {
            let cookie = delete_cookie(&Cookeys::AccessToken);
            cookies.remove(cookie);
            return Err((AuthErrors::InvalidToken, Some(cookies)));
        }
    };

    Ok((token_data, cookies))
}

impl IntoResponse for AuthErrors {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Invalid Token"),
                Some(403),
            )
            .into_response(),
            Self::MissingToken => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("No token was provided"),
                Some(403),
            )
            .into_response(),
            Self::FaildExtractingCookies => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Faild parsing cookies"),
                Some(500),
            )
            .into_response(),
            Self::InsufficientLevel => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("You level is too low"),
                Some(403),
            )
            .into_response(),
            Self::GuestRequired => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Need to be guest to access this route"),
                Some(403),
            )
            .into_response(),
            Self::AuthErrorWith200 => ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Need to be guest to access this route"),
                Some(200),
            )
            .into_response(),
        }
    }
}
