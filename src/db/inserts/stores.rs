use super::prelude::*;
use crate::{db::models::Store, prelude::*};

type InsertStoreResult = Result<Store>;

pub async fn new_store(
    db: &DBExtension,
    name: String,
    description: String,
    email: String,
    location: String,
) -> InsertStoreResult {
    let mut store = Store::new(name, description, email, location);

    let res = db
        .stores
        .insert_one(&store, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(Error::Static("TODO"));
        }
    };

    store.update_id(id);

    Ok(store)
}
