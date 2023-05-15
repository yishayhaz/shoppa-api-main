use super::prelude::*;
use crate::{db::models::Store, prelude::*};
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
    T::Error: Into<Error>,
{
    let store = store.try_into().map_err(|e| e.into())?;

    new_store(db, store).await
}
