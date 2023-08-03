use super::super::middlewares;
use axum::{middleware, routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    Router::new().route("/", routing::get(routes::get_invoices))
}
