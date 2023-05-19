use crate::helpers::MAX_IMAGE_SIZE;
use axum::{extract::DefaultBodyLimit, routing, Router};
mod get;
mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(manage::get_stores))
        .route("/", routing::post(manage::create_new_store))
        .route("/count", routing::get(manage::get_stores_count))
        .route("/autocomplete", routing::get(get::get_stores_autocomplete))
        .route("/:store_oid", routing::patch(manage::update_store))
        .layer(DefaultBodyLimit::max(MAX_IMAGE_SIZE * 2 + 1024))
        .route("/:store_oid", routing::get(manage::get_store_by_id))
        .route("/:store_oid/locations", routing::post(manage::add_store_location))
        .route("/:store_oid/locations/:location_oid", routing::delete(manage::delete_store_location))
        .route("/:store_oid/locations/:location_oid", routing::put(manage::update_store_location))
}
