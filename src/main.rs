use axum::{Extension, Router};
use dotenv::dotenv;
use shoppa_api::{
    api, db,
    helpers::{env::ENV_VARS, security::get_cors_layer, setup, self},
    services::file_storage,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "shoppa_api=debug,tower_http=debug".into()),
        )
        .init();

    dotenv().ok();

    ENV_VARS.validate();

    let mongo_client = db::connect().await.unwrap();

    let file_storge_client = Arc::new(file_storage::connect().await);

    let db_collections = Arc::new(db::DBCollections::new(mongo_client, &ENV_VARS.DB_NAME));

    // db_collections.create_indexes().await;

    let app = Router::new()
        .nest("/api/v1", api::v1::router())
        .layer(Extension(file_storge_client))
        .layer(Extension(db_collections))
        .layer(CookieManagerLayer::new())
        .layer(get_cors_layer())
        .layer(TraceLayer::new_for_http());

    let address = format!("{}:{}", &ENV_VARS.HOST, &ENV_VARS.PORT);

    println!("Listening on http://{}", address);

    let _ = axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(setup::shutdown_signal())
        .await;
}
