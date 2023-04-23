use axum::{routing, Router};

mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
    .route("/products", routing::get(manage::search_products))
    .route("/stores", routing::get(manage::search_stores))
}
