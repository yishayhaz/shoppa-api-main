use axum::{Router, routing};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::patch(routes::complete_registration))
}