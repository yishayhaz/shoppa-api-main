mod items;
mod routes;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/count", routing::get(routes::products_count))
        .nest("/:product_id/items", items::router())
        .route(
            "/:product_id/view",
            routing::put(routes::add_view_to_product),
        )
        .route("/:product_id", routing::get(routes::get_product))
        .route(
            "/autocomplete",
            routing::get(routes::products_autocomplete),
        )
        .route("/infinite", routing::get(routes::get_products_infinite))
        .route("/", routing::get(routes::get_products))
}
