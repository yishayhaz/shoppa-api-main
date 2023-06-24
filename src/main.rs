use axum::{Extension, Router};
use dotenv::dotenv;
use shoppa_api::{
    api,
    helpers::{env::ENV_VARS, security::get_cors_layer, setup},
};
use shoppa_core::{
    db::DBConection,
    email_sender::{EmailAddress, EmailClient},
    file_storage::StorageClient,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use validator::Validate;

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

    ENV_VARS
        .validate()
        .map_err(|e| panic!("ENV validation failed: \n{:?}", e))
        .unwrap();

    let storge_client = Arc::new(StorageClient::connect().await);

    let email_client = Arc::new(EmailClient::new(EmailAddress::new(
        "api@shoppa.co.il".to_string(),
        Some("API".to_string()),
    )));

    let db = Arc::new(DBConection::connect().await.unwrap());

    let app = Router::new()
        .nest("/api/v1", api::v1::router())
        .nest("/api/management", api::management::router())
        .nest("/api/stores", api::stores::router())
        .layer(Extension(email_client))
        .layer(Extension(storge_client))
        .layer(Extension(db))
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
