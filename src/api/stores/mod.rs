mod handlers;
mod middlewares;
use axum::{middleware, Router};

pub fn router() -> Router {
    Router::new()
        .nest("/me", handlers::me::router())
        .nest("/logout", handlers::logout::router())
        .nest("/products", handlers::products::router())
        .nest("/variants", handlers::variants::router())
        .nest("/store", handlers::store::router())
        .nest("/invoices", handlers::invoices::router())
        .nest("/orders", handlers::orders::router())
        .route_layer(middleware::from_fn(middlewares::login_required))
        .nest("/login", handlers::login::router())
        .nest("/registration", handlers::registration::router())
}
