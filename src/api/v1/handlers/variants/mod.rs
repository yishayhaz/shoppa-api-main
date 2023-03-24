use axum::{routing, Router};

mod manage;
mod types;

pub fn router() -> Router {
    Router::new().route("/", routing::post(manage::create_new_variant))
}
