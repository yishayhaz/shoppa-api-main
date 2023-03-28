mod types;
mod manage;

use axum::Router;

pub fn router() -> Router {
    Router::new()
}
