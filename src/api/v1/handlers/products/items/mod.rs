mod types;
mod add;

use axum::{Router, routing};

pub fn router() -> Router {
    Router::new()
        .route("/", routing::post(add::add_product_item))
}
