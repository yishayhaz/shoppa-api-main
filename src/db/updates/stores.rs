use crate::{
    db::models::{self, EmbeddedDocument, Store},
    helpers::types::DBExtension,
    prelude::*,
};
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneAndUpdateOptions;

type UpdateStoreResult = Result<Option<Store>>;

async fn private_update_store(
    db: &DBExtension,
    filter: Document,
    update: Document,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateStoreResult {
    let store = db
        .stores
        .find_one_and_update(filter, update, option)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(store)
}

pub async fn update_store(
    db: &DBExtension,
    store_id: &ObjectId,
    store_logo: Option<models::FileDocument>,
    store_banner: Option<models::FileDocument>,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateStoreResult {
    let filters = doc! {
        "_id": store_id
    };

    let mut update = doc! {};

    if let Some(store_logo) = store_logo {
        update.insert(Store::fields().logo, store_logo.into_bson()?);
    }

    if let Some(store_banner) = store_banner {
        update.insert(Store::fields().banner, store_banner.into_bson()?);
    }

    let update = doc! {
        "$set": update
    };

    private_update_store(db, filters, update, option).await
}