pub mod aggregations;
pub mod inserts;
pub mod models;
pub mod populate;
pub mod queries;
mod stores;
mod products;
mod categories;
pub mod updates;
use std::str::FromStr;

pub use products::*;
pub use stores::*;
pub use categories::*;

use crate::helpers::env::ENV_VARS;
use bson::{doc, Bson};
use models::DBModel;
use mongodb::{
    error::Error,
    options::{ClientOptions, ResolverConfig},
    Client, Collection,
};
use serde::Deserialize;
use serde_repr::{Deserialize_repr, Serialize_repr};

pub async fn connect() -> Result<Client, Error> {
    let options = ClientOptions::parse_with_resolver_config(
        &ENV_VARS.MONGODB_URI,
        ResolverConfig::cloudflare(),
    )
    .await?;

    let client = Client::with_options(options)?;

    Ok(client)
}

pub struct DBCollections {
    pub db: mongodb::Database,
    pub users: Collection<models::User>,
    pub stores: Collection<models::Store>,
    pub products: Collection<models::Product>,
    pub contact_us_form: Collection<models::ContactUsForm>,
    pub news_letter_subscribers: Collection<models::NewsLetterSubscriber>,
    pub site_visits: Collection<models::SiteVisit>,
    pub variants: Collection<models::Variants>,
    pub categories: Collection<models::Categories>,
}

impl DBCollections {
    pub fn new(client: Client, db_name: &str) -> Self {
        let db = client.database(db_name);

        let users = db.collection(models::User::get_collection_name());
        let stores = db.collection(models::Store::get_collection_name());
        let products = db.collection(models::Product::get_collection_name());
        let contact_us_form = db.collection(models::ContactUsForm::get_collection_name());
        let news_letter_subscribers =
            db.collection(models::NewsLetterSubscriber::get_collection_name());
        let site_visits = db.collection(models::SiteVisit::get_collection_name());
        let variants = db.collection(models::Variants::get_collection_name());
        let categories = db.collection(models::Categories::get_collection_name());

        Self {
            db,
            users,
            stores,
            products,
            contact_us_form,
            news_letter_subscribers,
            site_visits,
            variants,
            categories,
        }
    }

    pub async fn create_indexes(&self) {
        Self::create_index(&self.users).await;
        Self::create_index(&self.stores).await;
        Self::create_index(&self.products).await;
        Self::create_index(&self.contact_us_form).await;
        Self::create_index(&self.news_letter_subscribers).await;
        Self::create_index(&self.site_visits).await;
        Self::create_index(&self.variants).await;
        Self::create_index(&self.categories).await;
    }

    pub async fn create_schames(&self) {
        self.create_schame::<models::Store>().await;
    }

    async fn create_index<Model>(collection: &Collection<Model>)
    where
        Model: DBModel,
    {
        let indexes = Model::get_indexes();

        let indexes_names = indexes
            .iter()
            .map(|index| {
                let option = &index.options;

                if let Some(option) = option {
                    if let Some(name) = &option.name {
                        name.clone()
                    } else {
                        "".to_string()
                    }
                } else {
                    "".to_string()
                }
            })
            .collect::<Vec<String>>();

        let mut indexes_to_drop: Vec<String> = Vec::new();

        let mut current_indexes = match collection.list_indexes(None).await {
            Ok(v) => v,
            Err(e) => {
                println!(
                    "Failed to list indexes for {}",
                    Model::get_collection_name()
                );
                println!("Error: {}", e);
                return;
            }
        };

        while current_indexes.advance().await.unwrap() {
            let index = current_indexes.deserialize_current().unwrap();

            if let Some(options) = index.options {
                if let Some(name) = options.name {
                    if !indexes_names.contains(&name.to_string()) && !(name == "_id_") {
                        indexes_to_drop.push(name.to_string());
                    }
                }
            }
        }

        for index in indexes_to_drop {
            let _ = collection.drop_index(&index, None).await;
            println!(
                "Dropped index {} for {}",
                index,
                Model::get_collection_name()
            );
        }

        if indexes.len() > 0 {
            // If the index alredy exists, it will be ignored
            // https://www.mongodb.com/community/forums/t/behavior-of-createindex-for-an-existing-index/2248/2
            let _ = collection.create_indexes(indexes, None).await.expect(
                format!("Faild to create {} indexes", Model::get_collection_name()).as_str(),
            );
            println!("Created indexes for {}", Model::get_collection_name());
        }
    }

    async fn create_schame<Model>(&self)
    where
        Model: DBModel,
    {
        if let Some(validator) = Model::collection_validator() {
            self.db
                .run_command(
                    doc! {"collMod": Model::get_collection_name(), "validator": validator},
                    None,
                )
                .await
                .expect(
                    format!("Faild to create {} schame", Model::get_collection_name()).as_str(),
                );
        }
    }
}

pub struct Pagination {
    pub page: i64,
    pub amount: i64,
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct Sorter<T: FromStr> {
    #[serde(rename = "sort_by")]
    pub sort_by: T,
    #[serde(rename = "sort_direction")]
    pub direction: SortDirection,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(i8)]
pub enum SortDirection {
    Ascending = 1,
    Descending = -1,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            page: 0,
            amount: 20,
            offset: 0,
        }
    }
}

impl Default for Sorter<String> {
    fn default() -> Self {
        Sorter {
            sort_by: String::from("created_at"),
            direction: SortDirection::Descending,
        }
    }
}

impl From<&SortDirection> for bson::Bson {
    fn from(sort_dir: &SortDirection) -> Self {
        match sort_dir {
            SortDirection::Ascending => Bson::Int32(1),
            SortDirection::Descending => Bson::Int32(-1),
        }
    }
}

impl Pagination {
    pub fn need_count(&self, current_amount: usize) -> bool {
        let current_amount = current_amount as i64;

        if current_amount < self.amount {
            return false;
        }

        true
    }

    pub fn calculate_total(&self, current_amount: usize) -> u64 {
        (self.offset + current_amount as i64) as u64
    }
}
