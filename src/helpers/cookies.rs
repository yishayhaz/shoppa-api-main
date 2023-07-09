use crate::{
    helpers::{env::ENV_VARS, types::Cookeys},
    prelude::*,
    tokens::{USER_TOKEN_MANAGER, UserTokenData},
};
use shoppa_core::{constans::MAX_COOKIE_EXP, db::models::User};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub trait CookieManager {
    fn set_cookie(&self, key: &Cookeys, value: String, exp: i64, http_only: bool);

    fn delete_cookie(&self, key: &Cookeys);

    fn set_access_cookie(&self, user: &User) -> Result<()>;

    fn get_access_cookie(&self) -> Result<Option<String>>;

    fn delete_access_cookie(&self);
}

impl CookieManager for Cookies {
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

    fn set_access_cookie(&self, user: &User) -> Result<()> {

        let login_token = USER_TOKEN_MANAGER.generate_token(user, None)?;

        self.set_cookie(&Cookeys::AccessToken, login_token, MAX_COOKIE_EXP, true);

        Ok(())
    }

    fn get_access_cookie(&self) -> Result<Option<String>> {
        let cookie = self.get(&Cookeys::AccessToken.to_string());

        let cookie = match cookie {
            Some(cookie) => cookie,
            None => return Ok(None),
        };

        let cookie = cookie.value().to_string();

        Ok(Some(cookie))
    }

    fn delete_access_cookie(&self) {
        self.delete_cookie(&Cookeys::AccessToken);
    }
}
