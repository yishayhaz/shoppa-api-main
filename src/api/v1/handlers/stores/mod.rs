use axum::{routing, Router};

mod manage;
mod types;
mod get;

pub fn router() -> Router {
    Router::new()
    .route("/", routing::get(manage::get_stores))
    .route("/", routing::post(manage::create_new_store))
    .route("/:store_oid", routing::get(manage::get_store_by_id))
    .route("/:store_oid", routing::patch(manage::update_store))
    .route("/count", routing::get(manage::get_stores_count))
    .route("/autocomplete", routing::get(get::get_stores_autocomplete))
}
