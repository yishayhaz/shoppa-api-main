mod auth;
mod new_auth;
mod anti_auth;
pub use new_auth::{login_required, CurrentUser, login_required_or_create_guest};
pub use anti_auth::guest_required;
pub use auth::{
    GetTokenForGetMe, GuestOnly, Level1Access, Level1AccessOrNone, Level2Access, Level3Access,
};
use crate::helpers::env::ENV_VARS;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

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
