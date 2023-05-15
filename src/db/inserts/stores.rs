use super::prelude::*;
use crate::{db::models::Store, prelude::*};

pub async fn new_store<T>(db: &DBExtension, store: T) -> Result<Store>
where
    T: Into<Store>,
{
    let mut store = store.into();

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

pub async fn try_new_store<T>(db: &DBExtension, store: T) -> Result<Store>
where
    T: TryInto<Store>,
{
    todo!()
}
