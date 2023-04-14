use axum::{Extension, Router};
use dotenv::dotenv;
use shoppa_api::{
    api, db,
    helpers::{env::ENV_VARS, security::get_cors_layer, setup},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    ENV_VARS.validate();

    let mongo_client = db::connect().await.unwrap();

    let db_collections = Arc::new(db::DBCollections::new(mongo_client, &ENV_VARS.DB_NAME));

    // db_collections.create_indexes().await;

    let app = Router::new()
        .nest("/api/v1", api::v1::router())
        .layer(Extension(db_collections))
        .layer(CookieManagerLayer::new())
        .layer(get_cors_layer());

    let address = format!("{}:{}", &ENV_VARS.HOST, &ENV_VARS.PORT);

    println!("Listening on http://{}", address);

    let _ = axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(setup::shutdown_signal())
        .await;
}
