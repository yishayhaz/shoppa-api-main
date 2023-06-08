mod manage;
mod types;
mod get;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::get(get::get_root_categories))
        .route("/:cat_oid", routing::get(get::get_inner_categories))
        .route(
            "/:cat_oid/:cat_oid",
            routing::get(get::get_inner_inner_categories),
        )
        .route("/info", routing::get(get::get_category_info))
}
