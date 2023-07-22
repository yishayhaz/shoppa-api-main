use crate::api::v1::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route(
            "/login",
            routing::post(routes::login)
                .route_layer(middleware::from_fn(middlewares::guest_required)),
        )
        .route(
            "/logout",
            routing::delete(routes::logout)
                .route_layer(middleware::from_fn(middlewares::login_required)),
        )
        .route(
            "/signup",
            routing::post(routes::signup)
                .route_layer(middleware::from_fn(middlewares::guest_required)),
        )
        .route("/me", routing::get(routes::get_me)
            .route_layer(middleware::from_fn(middlewares::login_required))
        )
}
