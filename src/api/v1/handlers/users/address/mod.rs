use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route("/:address_oid", routing::patch(routes::edit_user_address))
        .route(
            "/:address_oid",
            routing::delete(routes::delete_user_address),
        )
        .route("/", routing::get(routes::get_user_addresses))
        .route("/", routing::post(routes::add_user_address))
}
