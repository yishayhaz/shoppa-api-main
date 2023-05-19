use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route(
            "/autocomplete",
            routing::get(routes::get_stores_autocomplete),
        )
        .route("/count", routing::get(routes::get_stores_count))
}
