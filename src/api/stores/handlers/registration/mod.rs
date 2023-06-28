use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::patch(routes::complete_registration))
        .route("/", routing::post(routes::validate_registration_token))
}
