use axum::{routing, Router};

mod cart;
mod get;
mod login;
mod login_;
mod password;
mod signup;
mod types;

pub fn router() -> Router {
    Router::new()
        .nest("/cart", cart::router())
        .route("/login", routing::post(login_::login))
        .route("/logout", routing::delete(login_::logout))
        .route("/sign-up", routing::post(signup::signup))
        .route("/sign-up/1", routing::post(signup::signup_level_1))
        .route("/me", routing::get(get::get_me))
        .route(
            "/update-password",
            routing::patch(password::change_password),
        )
}
