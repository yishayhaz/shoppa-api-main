use super::super::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::patch(routes::complete_registration))
        .route("/", routing::post(routes::validate_registration_token))
        .route_layer(middleware::from_fn(middlewares::guest_required))
}
