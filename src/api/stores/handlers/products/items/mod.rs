mod routes;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/:item-id", routing::patch(routes::edit_product_item))
        .route("/:item-id", routing::delete(routes::delete_product_item))
        .route("/", routing::post(routes::add_product_item))
}
