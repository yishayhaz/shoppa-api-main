mod items;
mod routes;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .nest("/:product_id/items", items::router())
        .route(
            "/:product_id/upload-files",
            routing::put(routes::upload_product_images),
        )
        .route("/", routing::post(routes::create_new_product))
}
