use axum::{routing, Router};
mod visitors;

pub fn router() -> Router {
    Router::new().route(
        "/visitor/counter",
        routing::put(visitors::add_new_visitor_to_counter),
    ).route(
        "/views/count",
        routing::get(visitors::get_views_count),
    )
}
