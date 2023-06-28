mod handlers;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/login", handlers::login::router())
        .nest("/registration", handlers::registration::router())
}
