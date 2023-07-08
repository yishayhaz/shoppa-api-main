use axum::{routing, Router};

mod get;
mod login;
mod password;
mod signup;
mod types;
mod cart;

pub fn router() -> Router {
    Router::new()
        .nest("/cart", cart::router())
        .route("/login", routing::post(login::login))
        .route("/logout", routing::delete(login::logout))
        .route("/sign-up", routing::post(signup::signup))
        .route("/sign-up/1", routing::post(signup::signup_level_1))
        .route("/me", routing::get(get::get_me))
        .route(
            "/update-password",
            routing::patch(password::change_password),
        )
}
