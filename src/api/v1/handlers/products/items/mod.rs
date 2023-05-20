mod routes;
mod types;

use axum::Router;

pub fn router() -> Router {
    Router::new()
}
