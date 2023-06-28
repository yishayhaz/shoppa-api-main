mod handlers;
mod middlewares;
use axum::{middleware, Router};

pub fn router() -> Router {
    Router::new()
        .nest("/products", handlers::products::router())
        .route_layer(middleware::from_fn(middlewares::login_required))
        .nest("/login", handlers::login::router())
        .nest("/registration", handlers::registration::router())
}
