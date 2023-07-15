use crate::api::v1::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(routes::get_full_cart))
        .route("/", routing::delete(routes::remove_product_from_cart))
        .route("/", routing::patch(routes::edit_product_in_cart))
        .route_layer(middleware::from_fn(middlewares::login_required))
        .route(
            "/",
            routing::post(routes::add_product_to_cart).layer(middleware::from_fn(
                middlewares::login_required_or_create_guest,
            )),
        )
}
