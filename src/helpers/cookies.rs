use crate::{
    helpers::{env::ENV_VARS, security, types::Cookeys},
    prelude::*,
};
use shoppa_core::{constans::MAX_COOKIE_EXP, db::models::User};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};

pub fn create_cookie<'a>(key: &'a Cookeys, value: String, exp: f64, http_only: bool) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.to_string(), value);

    cookie.set_http_only(http_only);

    cookie.set_secure(ENV_VARS.is_production());

    cookie.set_max_age(Duration::seconds_f64(exp));

    cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

    cookie.set_path("/");

    cookie
}

pub fn delete_cookie<'a>(key: &'a Cookeys) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.to_string(), "");

    cookie.set_max_age(Duration::seconds_f64(0.0));

    cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

    cookie.set_path("/");

    cookie
}

pub fn set_access_cookie(cookies: &Cookies, user: &User) -> Result<()> {
    let login_token = security::generate_login_token(&user)?;

    let access_cookie = create_cookie(&Cookeys::AccessToken, login_token, MAX_COOKIE_EXP, true);

    cookies.add(access_cookie);

    Ok(())
}
