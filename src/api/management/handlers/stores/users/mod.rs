use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
}
