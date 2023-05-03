mod get;
mod items;
mod manage;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/count", routing::get(get::products_count))
        .nest("/:product_id/items", items::router())
        .route("/:product_id/view", routing::put(manage::add_view_to_product))
        .route("/:product_id", routing::get(get::get_product))
        .route("/autocomplete", routing::get(get::products_names_for_autocomplete))
        .route("/infinite", routing::get(get::get_products))
        .route("/search", routing::get(get::get_products))
        .route("/", routing::post(manage::create_new_product))
}
