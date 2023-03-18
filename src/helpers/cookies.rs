use crate::helpers::{env::EnvVars, types::Cookeys};
use tower_cookies::{cookie::time::Duration, Cookie};

pub fn create_cookie<'a>(
    key: &'a Cookeys,
    value: String,
    exp: f64,
    http_only: bool,
) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.get(), value);

    cookie.set_http_only(http_only);

    cookie.set_secure(EnvVars::is_production());

    cookie.set_max_age(Duration::seconds_f64(exp));

    cookie.set_domain(EnvVars::COOKIE_DOMAIN.get());

    cookie.set_path("/");

    cookie
}

pub fn delete_cookie<'a>(
    key: &'a Cookeys,
) -> Cookie<'a> {
    let mut cookie = Cookie::new(key.get(), "");

    cookie.set_max_age(Duration::seconds_f64(0.0));

    cookie.set_domain(EnvVars::COOKIE_DOMAIN.get());

    cookie.set_path("/");

    cookie
}
