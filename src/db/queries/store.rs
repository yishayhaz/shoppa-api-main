use super::prelude::*;
use crate::prelude::*;

type GetStoreResult = Result<Option<models::Store>>;
// Stores as many stores
type GetStoresResult = Result<Vec<models::Store>>;

pub async fn get_stores(db: &DBExtension) -> GetStoresResult {
    let cursor = db
        .stores
        .find(None, None)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    let stores = consume_cursor(cursor)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(stores)
}

async fn get_store(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetStoreResult {
    let store = db
        .stores
        .find_one(filter, option)
        .await
        .map_err(|e| Error::DBError(("stores", e)))?;

    Ok(store)
}

pub async fn get_store_by_id(db: &DBExtension, id: &ObjectId) -> GetStoreResult {
    let filter = doc! {
        "_id": id,
    };

    get_store(db, filter, None).await
}
