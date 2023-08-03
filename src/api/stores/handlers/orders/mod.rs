use super::super::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/:order_oid", routing::patch(routes::update_order))
        .route("/:order_oid", routing::get(routes::get_order))
        .route("/", routing::get(routes::get_orders))
}
