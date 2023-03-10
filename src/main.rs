use axum::Router;
use dotenv::dotenv;
use shopa_api::{api, db, helpers::env::EnvVars};

#[tokio::main]
async fn main() {
    dotenv().ok();

    EnvVars::validate();

    let db_client = db::connect().await.expect("Failed to connect to database");

    let _data_base = db_client.database(&EnvVars::DB_NAME.get());

    let app = Router::new().nest("/api/v1", api::v1::router());

    let address = format!("{}:{}", EnvVars::HOST.get(), EnvVars::PORT.get());

    println!("Listening on http://{}", address);

    let _ = axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await;
}
