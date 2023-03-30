use super::prelude::*;
use crate::db::models::Store;

type InsertStoreResult = Result<Store, InsertDocumentErrors>;

pub async fn new_store(
    db: &DBExtension,
    name: String,
    email: String,
    location: String,
) -> InsertStoreResult{


    let mut store = Store::new(
        name,
        email,
        location
    );

    let res = match db
        .stores
        .insert_one(&store, None)
        .await
    {
        Ok(v) => v,
        Err(err) => return Err(extract_insert_document_error(*err.kind)),
    };

    let id = match res.inserted_id.as_object_id() {
        Some(obi) => obi,
        None => {
            return Err(InsertDocumentErrors::UnknownError);
        }
    };

    store.update_id(id);

    Ok(store)

}