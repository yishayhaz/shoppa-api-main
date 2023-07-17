use crate::helpers::env::ENV_VARS;
use axum::Router;
mod handlers;

pub fn router() -> Router {
    return Router::new()
        .nest("/stores", handlers::stores::router())
        .nest("/products", handlers::products::router())
        .nest("/variants", handlers::variants::router())
        .nest("/categories", handlers::categories::router());
}
