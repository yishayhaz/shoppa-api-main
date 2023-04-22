use super::prelude::*;

type GetStoreResult = Result<Option<models::Store>, Response>;

type GetStoresResult = Result<Vec<models::Store>, Response>;

pub async fn get_stores(
    db: &DBExtension,
) -> GetStoresResult {
    let cursor = match db.stores.find(None, None).await {
        Ok(cursor) => cursor,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching stores"),
                Some(500),
            )
            .into_response())
        }
    };

    let stores = match consume_cursor(cursor).await {
        Ok(stores) => stores,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching stores"),
                Some(500),
            )
            .into_response())
        }
    };

    Ok(stores)
}

async fn get_store(
    db: &DBExtension,
    filter: Document,
    option: Option<FindOneOptions>,
) -> GetStoreResult {
    let store = match db.stores.find_one(filter, option).await {
        Ok(store) => store,
        Err(_) => {
            return Err(ResponseBuilder::<u16>::error(
                // TODO add error code here
                "",
                None,
                Some("Internal Server Error while fetching store"),
                Some(500),
            )
            .into_response())
        }
    };

    Ok(store)
}

pub async fn get_store_by_id(db: &DBExtension, id: &ObjectId) -> GetStoreResult {
    let filter = doc! {
        "_id": id,
    };

    get_store(db, filter, None).await
}

pub async fn get_store_by_name(db: &DBExtension, name: String) -> GetStoreResult {
    let filter = doc! {
        "name": name,
    };

    get_store(db, filter, None).await
}