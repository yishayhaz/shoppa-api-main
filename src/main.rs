use axum::{Extension, Router};
use dotenv::dotenv;
use shopa_api::{api, db, helpers::env::EnvVars};
use std::sync::Arc;


#[tokio::main]
async fn main() {
    dotenv().ok();

    EnvVars::validate();

    let mongo_client = db::connect().await.unwrap();

    let db_collections = Arc::new(db::DBCollections::new(mongo_client, EnvVars::DB_NAME.get()));

    let app = Router::new()
        .nest("/api/v1", api::v1::router())
        .layer(Extension(db_collections));

    let address = format!("{}:{}", EnvVars::HOST.get(), EnvVars::PORT.get());

    println!("Listening on http://{}", address);

    let _ = axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
