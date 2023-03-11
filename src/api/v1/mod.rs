use axum::Router;

mod handlers;

pub fn router() -> Router {
    Router::new()
        .nest("/users", handlers::users::router())
        .nest("/products", handlers::products::router())
        .nest("/categories", handlers::categories::router())
}
