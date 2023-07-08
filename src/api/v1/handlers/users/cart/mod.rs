use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(routes::get_full_cart))
        .route("/", routing::post(routes::add_product_to_cart))
        .route("/", routing::delete(routes::remove_product_from_cart))
        .route("/", routing::patch(routes::edit_product_in_cart))
}
