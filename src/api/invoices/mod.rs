use axum::{routing, Router};
mod routes;
mod types;

pub fn router() -> Router {
    return Router::new().route(
        "/:token/reciept/:order_oid",
        routing::post(routes::update_mail_sent),
    );
}
