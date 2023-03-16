pub mod models;
pub mod queries;
pub mod updates;
use crate::helpers::env::EnvVars;
use models::DBModel;
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};

pub async fn connect() -> Result<Client, Error> {
    let options = ClientOptions::parse_with_resolver_config(
        &EnvVars::MONGODB_URI.get(),
        ResolverConfig::cloudflare(),
    )
    .await?;

    let client = Client::with_options(options)?;

    Ok(client)
}

pub struct DBCollections {
    pub users: Collection<models::User>,
    pub stores: Collection<models::Store>,
    pub products: Collection<models::Product>,
    pub contact_us_form: Collection<models::ContactUsForm>,
}

impl DBCollections {
    pub fn new(client: Client, db_name: String) -> Self {
        let db = client.database(&db_name);

        let users = db.collection(models::User::get_collection_name());
        let stores = db.collection(models::Store::get_collection_name());
        let products = db.collection(models::Product::get_collection_name());
        let contact_us_form = db.collection(models::ContactUsForm::get_collection_name());

        Self {
            users,
            stores,
            products,
            contact_us_form
        }
    }
}
