mod routes;
mod types;


use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(routes::create_new_catagorie))
}