use axum::{routing, Router};
mod manage;
mod types;

pub fn router() -> Router {
    Router::new()
    .route("/", routing::post(manage::contact_us_request))
    .route("/", routing::get(manage::get_contact_us))
    .route("/:form_oid", routing::put(manage::update_status))
}
