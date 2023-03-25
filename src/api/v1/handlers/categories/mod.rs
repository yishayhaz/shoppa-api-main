mod manage;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(manage::create_new_root_catagorie))
        .route("/inner", routing::post(manage::create_new_inner_catagorie))
        .route(
            "/inner/inner",
            routing::post(manage::create_new_inner_inner_catagorie),
        )
}
