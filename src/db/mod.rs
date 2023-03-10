pub mod models;
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client,
};

use crate::helpers::env::EnvVars;

pub async fn connect() -> Result<Client, Error> {
    let options = ClientOptions::parse_with_resolver_config(
        &EnvVars::MONGODB_URI.get(),
        ResolverConfig::cloudflare(),
    )
    .await?;

    let client = Client::with_options(options)?;

    Ok(client)
}
