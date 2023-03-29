mod auth;
mod ip_address;

pub use auth::{GuestOnly, GetTokenForGetMe, Level1Access, Level1AccessOrNone, Level2Access, Level3Access};
pub use ip_address::ClientIpAddress;


use crate::helpers::{
    env::EnvVars
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};

pub struct OnlyInDev();

#[async_trait]
impl<S> FromRequestParts<S> for OnlyInDev
where
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if EnvVars::is_production() {
            return Err("?")
        }

        Ok(OnlyInDev())
    }
}