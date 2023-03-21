use crate::helpers::env::EnvVars;
use http::{request::Parts as RequestParts, HeaderValue};
use tower_http::cors::{CorsLayer, AllowOrigin};

pub fn get_cors_layer() -> CorsLayer {
    let methods = [
        "POST".parse().unwrap(),
        "GET".parse().unwrap(),
        "PUT".parse().unwrap(),
        "PATCH".parse().unwrap(),
        "DELETE".parse().unwrap(),
        "OPTIONS".parse().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_methods(methods)
        .allow_credentials(true)
        .allow_origin(AllowOrigin::predicate(
        |origin: &HeaderValue, _request_parts: &RequestParts| {
            origin.as_bytes().ends_with(EnvVars::COOKIE_DOMAIN.get().as_bytes())
        },
    ));

    cors
}
