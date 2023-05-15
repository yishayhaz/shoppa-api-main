mod manage;
mod types;
mod get;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        // create categories
        .route("/", routing::post(manage::create_new_root_catagorie))
        .route(
            "/",
            routing::delete(manage::delete_category_by_ids),
        )
        .route("/", routing::put(manage::update_category_by_ids))
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
        .route("/info", routing::get(get::get_category_info))
}
