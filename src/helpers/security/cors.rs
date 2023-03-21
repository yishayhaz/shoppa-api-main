use crate::helpers::env::EnvVars;
use tower_http::cors::CorsLayer;

pub fn get_cors_layer() -> CorsLayer {
    let mut schema = "http://";

    if EnvVars::is_production() {
        schema = "https://";
    };

    let origins = [format!("{}{}", schema, EnvVars::COOKIE_DOMAIN.get())
        .parse()
        .unwrap()];

    let methods = [
        "POST".parse().unwrap(),
        "GET".parse().unwrap(),
        "PUT".parse().unwrap(),
        "PATCH".parse().unwrap(),
        "DELETE".parse().unwrap(),
    ];

    CorsLayer::new()
        .allow_methods(methods)
        .allow_credentials(true)
        .allow_origin(origins)
}
