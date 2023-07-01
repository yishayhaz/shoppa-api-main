use axum::{routing, Router};

mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/by-ids", routing::get(routes::get_variants_by_ids))
        .route(
            "/autocomplete",
            routing::get(routes::autocomplete_variants),
        )
}

