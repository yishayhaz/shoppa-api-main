mod manage;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new().route("/", routing::post(manage::create_new_product))
}
