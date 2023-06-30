use axum::{routing, Router};
mod routes;

pub fn router() -> Router {
    Router::new().route("/", routing::get(routes::get_me))
}
