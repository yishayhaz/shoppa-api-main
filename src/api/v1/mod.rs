use axum::Router;

mod handlers;

pub fn router() -> Router {
    Router::new().nest("/users", handlers::users::router())
}
