mod types;
mod routes;

use axum::{Router, routing};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(routes::add_product_item))
        .route("/:item-id", routing::patch(routes::edit_product_item))
}