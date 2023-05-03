use axum::{routing, Router};

mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
    .route("/", routing::get(manage::get_stores))
    .route("/", routing::post(manage::create_new_store))
    .route("/:store_oid", routing::get(manage::get_store_by_id))
    .route("/count", routing::get(manage::get_stores_count))
}
