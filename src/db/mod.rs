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

        Self {
            users,
            stores,
            products,
            contact_us_form,
            news_letter_subscribers,
            site_visits,
        }
    }

    pub async fn create_indexs(&self) {
        let users_indexes = models::User::get_indexes();

        if users_indexes.len() > 0 {
            let _ = self.users.drop_indexes(None).await;

            let _ = self
                .users
                .create_indexes(users_indexes, None)
                .await
                .expect("Faild to create user indexes");
        }

        let stores_indexes = models::Store::get_indexes();

        if stores_indexes.len() > 0 {
            let _ = self.stores.drop_indexes(None).await;

            let _ = self
                .stores
                .create_indexes(stores_indexes, None)
                .await
                .expect("Faild to create store indexes");
        }

        let products_indexes = models::Product::get_indexes();

        if products_indexes.len() > 0 {
            let _ = self.products.drop_indexes(None).await;

            let _ = self
                .products
                .create_indexes(products_indexes, None)
                .await
                .expect("Faild to create product indexes");
        }

        let contact_us_form_indexes = models::ContactUsForm::get_indexes();

        if contact_us_form_indexes.len() > 0 {
            let _ = self.contact_us_form.drop_indexes(None).await;

            let _ = self
                .contact_us_form
                .create_indexes(contact_us_form_indexes, None)
                .await
                .expect("Faild to create contact us form indexes");
        }

        let news_letter_subscribers_indexes = models::NewsLetterSubscriber::get_indexes();

        if news_letter_subscribers_indexes.len() > 0 {
            let _ = self.news_letter_subscribers.drop_indexes(None).await;

            let _ = self
                .news_letter_subscribers
                .create_indexes(news_letter_subscribers_indexes, None)
                .await
                .expect("Faild to create news letter subscribers indexes");
        }

        let site_visit_indexes = models::SiteVisit::get_indexes();

        if site_visit_indexes.len() > 0 {
            let _ = self.site_visits.drop_indexes(None).await;

            let _ = self
                .site_visits
                .create_indexes(site_visit_indexes, None)
                .await
                .expect("Faild to create site visits indexes");
        }
    }
}
