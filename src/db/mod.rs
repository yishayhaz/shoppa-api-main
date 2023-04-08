pub mod aggregations;
pub mod inserts;
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
use bson::{Bson, doc, Document};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde::Deserialize;

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
    pub news_letter_subscribers: Collection<models::NewsLetterSubscriber>,
    pub site_visits: Collection<models::SiteVisit>,
    pub variants: Collection<models::Variants>,
    pub product_items: Collection<models::ProductItems>,
    pub categories: Collection<models::Categories>,
}

impl DBCollections {
    pub fn new(client: Client, db_name: String) -> Self {
        let db = client.database(&db_name);

        let users = db.collection(models::User::get_collection_name());
        let stores = db.collection(models::Store::get_collection_name());
        let products = db.collection(models::Product::get_collection_name());
        let contact_us_form = db.collection(models::ContactUsForm::get_collection_name());
        let news_letter_subscribers =
            db.collection(models::NewsLetterSubscriber::get_collection_name());
        let site_visits = db.collection(models::SiteVisit::get_collection_name());
        let variants = db.collection(models::Variants::get_collection_name());
        let product_items = db.collection(models::ProductItems::get_collection_name());
        let categories = db.collection(models::Categories::get_collection_name());

        Self {
            users,
            stores,
            products,
            contact_us_form,
            news_letter_subscribers,
            site_visits,
            variants,
            product_items,
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
        Self::create_index(&self.product_items).await;
        Self::create_index(&self.categories).await;
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
}

pub struct Pagination {
    pub page: i64,
    pub amount: i64,
    pub offset: i64,
}

#[derive(Deserialize)]
pub struct Sorter {
    #[serde(rename="sort_field")]
    pub field: String,
    #[serde(rename="sort_direction")]
    pub direction: SortDireaction,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone)]
#[repr(i8)]
pub enum SortDireaction {
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

impl Default for Sorter {
    fn default() -> Self {
        Sorter {
            field: String::from("created_at"),
            direction: SortDireaction::Descending,
        }
    }
}


impl SortDireaction {
    pub fn into_bson(&self) -> bson::Bson {
        match self {
            Self::Ascending => Bson::Int32(1),
            Self::Descending => Bson::Int32(-1)
        }
    }
}

impl Into<Document> for Sorter {
    fn into(self) -> Document {
        doc!{
            self.field: self.direction.into_bson()
        }
    }
}