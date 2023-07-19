use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(routes::create_store_user))
        .route("/:store_id", routing::get(routes::get_store_users))
}
