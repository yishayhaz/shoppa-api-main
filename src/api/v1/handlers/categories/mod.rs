mod manage;
mod types;
mod get;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        // create categories
        .route("/", routing::post(manage::create_new_root_catagorie))
        .route("/:cat_oid", routing::post(manage::create_new_inner_catagorie))
        .route(
            "/:cat_oid/:cat_oid",
            routing::post(manage::create_new_inner_inner_catagorie),
        )
        // get categories
        .route("/", routing::get(get::get_root_categories))
        .route("/:cat_oid", routing::get(get::get_inner_categories))
        .route(
            "/:cat_oid/:cat_oid",
            routing::get(get::get_inner_inner_categories),
        )
}
