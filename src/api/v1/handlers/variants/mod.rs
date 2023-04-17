use axum::{routing, Router};

mod get;
mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(manage::create_new_variant))
        .route("/", routing::get(get::get_variants))
}
