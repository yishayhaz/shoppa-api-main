use crate::{
    db::models::{self, EmbeddedDocument, Store, StoreLocation},
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
    store_logo: Option<Option<models::FileDocument>>,
    store_banner: Option<Option<models::FileDocument>>,
    name: Option<String>,
    description: Option<String>,
    slogan: Option<Option<String>>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    legal_id: Option<String>,
    business_type: Option<models::StoreBusinessType>,
    business_name: Option<String>,
    option: Option<FindOneAndUpdateOptions>,
) -> UpdateStoreResult {
    let filters = doc! {
        "_id": store_id
    };

    let mut update = doc! {};

    if let Some(store_logo) = store_logo {
        if let Some(store_logo) = store_logo {
            update.insert(Store::fields().logo, store_logo.into_bson()?);
        } else {
            update.insert::<_, Option<&str>>(Store::fields().logo, None);
        }
    }

    if let Some(store_banner) = store_banner {
        if let Some(store_banner) = store_banner {
            update.insert(Store::fields().banner, store_banner.into_bson()?);
        } else {
            update.insert::<_, Option<&str>>(Store::fields().banner, None);
        }
    }

    if let Some(name) = name {
        update.insert(Store::fields().name, name);
    }

    if let Some(description) = description {
        update.insert(Store::fields().description, description);
    }

    if let Some(slogan) = slogan {
        if let Some(slogan) = slogan {
            update.insert(Store::fields().slogan, slogan);
        } else {
            update.insert::<_, Option<&str>>(Store::fields().slogan, None);
        }
    }

    if let Some(contact_email) = contact_email {
        update.insert(Store::fields().contact(true).email, contact_email);
    }

    if let Some(contact_phone) = contact_phone {
        update.insert(Store::fields().contact(true).phone, contact_phone);
    }

    if let Some(legal_id) = legal_id {
        update.insert(Store::fields().legal_information(true).legal_id, legal_id);
    }

    if let Some(business_type) = business_type {
        update.insert(
            Store::fields().legal_information(true).business_type,
            business_type,
        );
    }

    if let Some(business_name) = business_name {
        update.insert(Store::fields().legal_information(true).name, business_name);
    }

    let update = doc! {
        "$set": update
    };

    private_update_store(db, filters, update, option).await
}

pub async fn add_store_locations(
    db: &DBExtension,
    store_id: &ObjectId,
    location: &StoreLocation,
) -> UpdateStoreResult {
    let filters = doc! {
        "_id": store_id
    };

    let update = doc! {
        "$push": {
            Store::fields().locations: location
        }
    };

    private_update_store(db, filters, update, None).await
}

pub async fn delete_store_location(
    db: &DBExtension,
    store_id: &ObjectId,
    location_id: &ObjectId,
) -> UpdateStoreResult {
    let filters = doc! {
        "_id": store_id
    };

    let update = doc! {
        "$pull": {
            Store::fields().locations: {
                "_id": location_id
            }
        }
    };

    private_update_store(db, filters, update, None).await
}
