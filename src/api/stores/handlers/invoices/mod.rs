use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new()
        .route(
            "/:invoice_oid/download",
            routing::get(routes::install_invoice),
        )
        .route("/", routing::get(routes::get_invoices))
}
