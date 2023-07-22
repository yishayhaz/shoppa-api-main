use crate::api::v1::middlewares;
use axum::{middleware, routing, Router};
mod cart;
mod password;
mod types;

pub fn router() -> Router {
    Router::new()
        .route(
            "/update-password",
            routing::patch(password::change_password),
        )
        .route_layer(middleware::from_fn(middlewares::login_required))
        .nest("/cart", cart::router())
}
