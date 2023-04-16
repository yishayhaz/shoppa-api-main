mod manage;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(manage::create_new_root_catagorie))
        .route("/:cat_oid", routing::post(manage::create_new_inner_catagorie))
        .route(
            "/:cat_oid/:cat_oid",
            routing::post(manage::create_new_inner_inner_catagorie),
        )
}
