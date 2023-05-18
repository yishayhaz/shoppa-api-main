use super::prelude::*;
use crate::{db::{models::{Store, StoreLocation}}, prelude::*};
use validator::Validate;

pub async fn new_store<T>(db: &DBExtension, store: T) -> Result<Store>
where
    T: Into<Store>,
{
    let mut store: Store = store.into();

    store.validate()?;

    let res = db
        .stores
        .insert_one(&store, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obj) => obj,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    store.update_id(id);

    Ok(store)
}

pub async fn add_store_location(
    db: &DBExtension,
    store_id: &ObjectId,
    location: &StoreLocation,
) -> Result<Store> {
    todo!("add_store_location");

    // let filters = doc! {
    //     "_id": store_id
    // };

    // let update = doc! {
    //     // "$push": {
    //     //     Store::fields().locations: location
    //     // }
    // };

    // let store = db
    //     .stores
    //     .find_one_and_update(filters, update, None)
    //     .await
    //     .map_err(|e| Error::DBError(("stores", e)))?;

    // Ok(store)
}