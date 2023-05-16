use axum::{routing, Router};

mod get;
mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
    .route("/", routing::get(get::get_variants))
    .route("/by-ids", routing::get(get::get_variants_by_ids))
        .route("/", routing::post(manage::create_new_variant))
        .route("/:variant_id", routing::get(get::get_variant_by_id))
        .route("/:variant_id", routing::put(manage::update_variant))
        .route("/:variant_id", routing::delete(manage::delete_variant))
        .route("/:variant_id/value", routing::post(manage::add_value_to_variant))
        .route("/:variant_id/value/:value_id", routing::put(manage::update_variant_value))
        .route("/:variant_id/value/:value_id", routing::delete(manage::delete_variant_value))
}
