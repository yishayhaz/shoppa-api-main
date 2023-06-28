use axum::Router;
mod routes;
mod types;

pub fn router() -> Router {
    Router::new().route("/", axum::routing::post(routes::login))
}
