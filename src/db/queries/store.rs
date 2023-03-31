use crate::{
    db::models,
    helpers::types::{DBExtension, ResponseBuilder},
};
use axum::response::IntoResponse;
use axum::response::Response;
use bson::{doc, oid::ObjectId, Document};
use mongodb::options::FindOneOptions;

type GetStoreResult = Result<Option<models::Store>, Response>;

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
