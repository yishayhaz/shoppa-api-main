use axum::{Extension, Router};
use dotenv::dotenv;
use shopa_api::{api, db, helpers::env::EnvVars};
use std::sync::Arc;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    dotenv().ok();

    EnvVars::validate();

    let mongo_client = db::connect().await.unwrap();

    let db_collections = Arc::new(db::DBCollections::new(mongo_client, EnvVars::DB_NAME.get()));

    db_collections.create_indexs().await;

    let app = Router::new()
        .nest("/api/v1", api::v1::router())
        .layer(Extension(db_collections))
        .layer(CookieManagerLayer::new());

    let address = format!("{}:{}", EnvVars::HOST.get(), EnvVars::PORT.get());

    println!("Listening on http://{}", address);

    let _ = axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
