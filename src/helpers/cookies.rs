use crate::{
    helpers::{env::ENV_VARS, types::Cookeys},
    prelude::*,
    tokens::{CHECKOUT_SESSION_TOKEN_MANAGER, USER_TOKEN_MANAGER},
};
use shoppa_core::{
    constans::MAX_COOKIE_EXP,
    db::models::{CheckOutSession, User},
};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub trait CookieManager {
    fn get_cookie(&self, key: &Cookeys) -> Option<Cookie<'_>>;

    fn set_cookie(&self, key: &Cookeys, value: String, exp: i64, http_only: bool);

    fn delete_cookie(&self, key: &Cookeys);

    fn set_access_cookie(&self, user: &User) -> Result<()> {
        let login_token = USER_TOKEN_MANAGER.generate_token(user, None)?;

        self.set_cookie(&Cookeys::AccessToken, login_token, MAX_COOKIE_EXP, true);

        Ok(())
    }

    fn get_access_cookie(&self) -> Option<String> {
        let cookie = self.get_cookie(&Cookeys::AccessToken);

        if let Some(cookie) = cookie {
            if cookie.value().is_empty() {
                return None;
            }
            return Some(cookie.value().to_string());
        }

        None
    }

    fn delete_access_cookie(&self) {
        self.delete_cookie(&Cookeys::AccessToken);
    }

    fn set_checkout_session_cookie(&self, checkout_session: &CheckOutSession) -> Result<()> {
        self.set_cookie(
            &Cookeys::CheckoutSession,
            CHECKOUT_SESSION_TOKEN_MANAGER.generate_token(checkout_session, None)?,
            // 30 minutes
            60 * 30,
            true,
        );

        Ok(())
    }

    fn get_checkout_session_cookie(&self) -> Option<String> {
        let cookie = self.get_cookie(&Cookeys::CheckoutSession);

        if let Some(cookie) = cookie {
            if cookie.value().is_empty() {
                return None;
            }
            return Some(cookie.value().to_string());
        }

        None
    }

    fn delete_checkout_session_cookie(&self) {
        self.delete_cookie(&Cookeys::CheckoutSession);
    }
}

impl CookieManager for Cookies {
    fn get_cookie(&self, key: &Cookeys) -> Option<Cookie<'_>> {
        self.get(&key.to_string())
    }

    fn set_cookie(&self, key: &Cookeys, value: String, exp: i64, http_only: bool) {
        let mut cookie = Cookie::new(key.to_string(), value);

        cookie.set_http_only(http_only);

        cookie.set_secure(ENV_VARS.is_production());

        cookie.set_max_age(Duration::seconds(exp as i64));

        cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

        cookie.set_path("/");

        self.add(cookie);
    }

    fn delete_cookie(&self, key: &Cookeys) {
        let mut cookie = Cookie::new(key.to_string(), "");

        cookie.set_max_age(Duration::seconds(0));

        cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

        cookie.set_path("/");

        self.add(cookie);
    }
}
