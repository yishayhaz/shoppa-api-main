use axum::Router;

mod handlers;

pub fn router() -> Router {
    Router::new().nest("/stores", handlers::stores::router())
}
