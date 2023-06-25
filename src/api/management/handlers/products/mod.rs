mod items;
mod routes;
mod types;
use axum::extract::DefaultBodyLimit;
use shoppa_core::constans::MAX_IMAGE_SIZE;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route(
            "/:product_id/upload-file",
            routing::put(routes::upload_product_images),
        )
        .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE + 1024))
        .nest("/:product_id/items", items::router())
        .route("/:product_id/delete-file/:file_id", routing::delete(routes::delete_product_file))
        .route("/:product_id", routing::patch(routes::edit_product))
        .route("/:product_id", routing::delete(routes::delete_product))
        .route("/:product_id", routing::get(routes::get_product))
        .route("/", routing::post(routes::create_new_product))
}
