use axum::{routing, Router};
mod routes;

pub fn router() -> Router {
    Router::new().route("/", routing::delete(routes::logout))
}
