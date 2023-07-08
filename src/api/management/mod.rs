use crate::helpers::env::ENV_VARS;
use axum::Router;
mod handlers;

pub fn router() -> Router {
    if ENV_VARS.is_production() || ENV_VARS.is_stage(){
        return Router::new();
    }
    Router::new()
        .nest("/stores", handlers::stores::router())
        .nest("/products", handlers::products::router())
        .nest("/variants", handlers::variants::router())
        .nest("/categories", handlers::categories::router())
}
