use axum::{extract::DefaultBodyLimit, routing, Router};
use shoppa_core::constans::MAX_IMAGE_SIZE;
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(routes::get_my_store))
        .route(
            "/assets",
            routing::put(routes::update_store_assets)
                .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE * 2 + 1024)),
        )
        .route("/", routing::patch(routes::update_store))
        .route("/", routing::get(routes::get_current_user_store))
        .route("/locations", routing::post(routes::add_store_locations))
        .route(
            "/locations/:location_oid",
            routing::delete(routes::delete_store_location),
        )
        .route(
            "/locations/:location_oid",
            routing::patch(routes::update_store_location),
        )
}
