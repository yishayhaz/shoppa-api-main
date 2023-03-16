use axum::Router;

mod handlers;

pub fn router() -> Router {
    Router::new()
        .nest("/users", handlers::users::router())
        .nest("/products", handlers::products::router())
        .nest("/categories", handlers::categories::router())
        .nest("/analytics", handlers::analytics::router())
        .nest("/blog", handlers::blog::router())
        .nest("/contact-us", handlers::contact_us::router())
}
