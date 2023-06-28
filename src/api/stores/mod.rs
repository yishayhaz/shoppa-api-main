mod handlers;
mod middlewares;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .nest("/login", handlers::login::router())
        .nest("/registration", handlers::registration::router())
        .nest("/products", handlers::products::router())
}
