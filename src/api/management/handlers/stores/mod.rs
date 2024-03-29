use axum::{extract::DefaultBodyLimit, routing, Router};
use shoppa_core::constans::MAX_IMAGE_SIZE;
mod routes;
mod types;
mod users;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:store_oid/assets",
            routing::put(routes::update_store_assets)
                .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE * 2 + 1024)),
        )
        .nest("/users", users::router())
        .route("/:store_oid", routing::patch(routes::update_store))
        .route("/:store_oid", routing::get(routes::get_store_by_id))
        .route("/", routing::post(routes::create_new_store))
        .route(
            "/:store_oid/locations",
            routing::post(routes::add_store_locations),
        )
        .route(
            "/:store_oid/locations/:location_oid",
            routing::delete(routes::delete_store_location),
        )
        .route(
            "/:store_oid/locations/:location_oid",
            routing::patch(routes::update_store_location),
        )
        .route("/", routing::get(routes::get_stores))
}
