mod get;
mod items;
mod manage;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .nest("/:product_id/items", items::router())
        .route("/", routing::post(manage::create_new_product))
        .route("/", routing::get(get::get_products))
        .route("/:product_id/view", routing::put(manage::add_view_to_product))
}
