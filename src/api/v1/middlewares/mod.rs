mod anti_auth;
mod auth;
mod checkout_session;
use crate::helpers::env::ENV_VARS;
pub use anti_auth::guest_required;
pub use auth::{
    guest_user_not_allowed, login_required, login_required_200, login_required_or_create_guest,
    CurrentUser,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
pub use checkout_session::{checkout_session_required, CurrentCheckOutSession};

pub struct OnlyInDev();

#[async_trait]
impl<S> FromRequestParts<S> for OnlyInDev
where
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if ENV_VARS.is_production() {
            return Err("?");
        }

        Ok(OnlyInDev())
    }
}
