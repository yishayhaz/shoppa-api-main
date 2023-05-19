use crate::helpers::env::ENV_VARS;
use axum::Router;
mod handlers;

pub fn router() -> Router {
    if ENV_VARS.is_production() {
        return Router::new();
    }
    Router::new().nest("/stores", handlers::stores::router())
}
