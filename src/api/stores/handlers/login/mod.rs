use super::super::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(routes::login))
        .route_layer(middleware::from_fn(middlewares::guest_required))
}
