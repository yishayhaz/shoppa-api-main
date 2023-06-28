mod routes;
mod items;
mod types;

use axum::extract::DefaultBodyLimit;
use shoppa_core::constans::MAX_IMAGE_SIZE;
use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
    .route(
        "/:product_id/upload-file",
        routing::put(routes::upload_product_asset),
    )
    .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE + 1024))
    .nest("/:product_id/items", items::router())
    .route("/:product_id/delete-file/:file_id", routing::delete(routes::delete_product_asset))
    .route("/:product_id", routing::patch(routes::edit_product))
    .route("/:product_id", routing::delete(routes::delete_product))
    .route("/:product_id", routing::get(routes::get_product))
    .route("/", routing::post(routes::create_new_product))
    .route("/", routing::get(routes::get_products))
}
