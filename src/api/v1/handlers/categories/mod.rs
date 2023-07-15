mod routes;
mod types;
use axum::{routing, Router};

pub fn router() -> Router {
    Router::new().route("/", routing::get(routes::get_catagories))
}
