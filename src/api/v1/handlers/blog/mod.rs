use axum::{routing, Router};

mod news_letter;
mod types;

pub fn router() -> Router {
    Router::new().route(
        "/sign-up",
        routing::post(news_letter::signup_to_news_letter),
    )
}
