use axum::{routing, Router};

mod cart;
mod get;
mod password;
mod types;

pub fn router() -> Router {
    Router::new()
        .nest("/cart", cart::router())
        .route("/me", routing::get(get::get_me))
        .route(
            "/update-password",
            routing::patch(password::change_password),
        )
}
