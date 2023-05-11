mod types;
mod manage;

use axum::{Router, routing};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(manage::add_product_item))
        .route("/:item-id", routing::patch(manage::edit_product_item))
}
