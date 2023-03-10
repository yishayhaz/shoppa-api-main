use axum::{Router, routing};

mod login;
mod signup;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/login", routing::post(login::login))
        .route("/logout", routing::delete(login::logout))
        .route("/sign-up/", routing::post(signup::signup))
        .route("/sign-up/1", routing::post(signup::signup_level_1))
}
