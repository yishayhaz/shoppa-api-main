use crate::{helpers::{env::ENV_VARS, types::{Cookeys, MAX_COOKIE_EXP}, security}, db::models::User};
use tower_cookies::{cookie::time::Duration, Cookie, Cookies};
use axum::response::Response;


pub fn create_cookie<'a>(
    key: &'a Cookeys,
    value: String,
    exp: f64,
    http_only: bool,
) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.get(), value);

    cookie.set_http_only(http_only);

    cookie.set_secure(ENV_VARS.is_production());

    cookie.set_max_age(Duration::seconds_f64(exp));

    cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

    cookie.set_path("/");

    cookie
}

pub fn delete_cookie<'a>(
    key: &'a Cookeys,
) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.get(), "");

    cookie.set_max_age(Duration::seconds_f64(0.0));

    cookie.set_domain(&ENV_VARS.COOKIE_DOMAIN);

    cookie.set_path("/");

    cookie
}


pub fn set_access_cookie(
    cookies: &Cookies,
    user: &User
) -> Result<(), Response>{

    let login_token = security::generate_login_token(&user)?;

    let access_cookie = create_cookie(
        &Cookeys::AccessToken,
        login_token,
        MAX_COOKIE_EXP,
        true
    );

    cookies.add(access_cookie);

    Ok(())
}

