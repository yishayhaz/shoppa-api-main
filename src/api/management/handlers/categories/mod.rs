mod routes;
mod types;

use axum::{routing, Router};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(routes::create_new_catagory))
        .route("/:category_id", routing::get(routes::get_category))
        .route("/:category_id", routing::patch(routes::edit_category))
}
