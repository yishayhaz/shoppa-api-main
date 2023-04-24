mod categories;
mod contact_us;
mod prelude;
mod products;
mod store;
mod users;
mod variants;

use bson::{Bson, Document};
pub use categories::*;
pub use contact_us::*;
pub use products::*;
pub use store::*;
pub use users::*;
pub use variants::*;

use axum::response::Response;
use futures_util::StreamExt;
use mongodb::{error::Error, Cursor};
use serde::Deserialize;

pub async fn consume_cursor<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<T>,
) -> Result<Vec<T>, Error> {
    let mut documents: Vec<T> = Vec::new();

    while cursor.advance().await? {
        documents.push(cursor.deserialize_current()?);
    }

    Ok(documents)
}

pub async fn convert_one_doc_cursor<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<Document>,
) -> Result<Option<T>, Error> {
    let doc = cursor.next().await.transpose()?;

    if let Some(doc) = doc {
        let doc = bson::from_bson::<T>(Bson::Document(doc))?;
        return Ok(Some(doc));
    }

    Ok(None)
}

pub type PaginatedResult<T> = Result<(Vec<T>, u64), Response>;
