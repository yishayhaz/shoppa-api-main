use axum::{routing, Router};

mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(routes::get_variants))
        .route("/by-ids", routing::get(routes::get_variants_by_ids))
        .route("/", routing::post(routes::create_new_variant))
        .route("/:variant_id", routing::get(routes::get_variant_by_id))
        .route("/:variant_id", routing::put(routes::update_variant))
        .route("/:variant_id", routing::delete(routes::delete_variant))
        .route(
            "/:variant_id/value",
            routing::post(routes::add_value_to_variant),
        )
        .route(
            "/:variant_id/value/:value_id",
            routing::put(routes::update_variant_value),
        )
        .route(
            "/:variant_id/value/:value_id",
            routing::delete(routes::delete_variant_value),
        )
}
