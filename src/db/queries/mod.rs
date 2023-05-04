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

use futures_util::{StreamExt, Stream};
use mongodb::{error::Error as MongoDBError, Cursor};
use serde::Deserialize;
use crate::prelude::*;

pub async fn consume_cursor<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<T>,
) -> StdResult<Vec<T>, MongoDBError> {
    let mut documents: Vec<T> = Vec::new();

    while cursor.advance().await? {
        documents.push(cursor.deserialize_current()?);
    }

    Ok(documents)
}

pub async fn convert_one_doc_cursor<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<Document>,
) -> StdResult<Option<T>, MongoDBError> {
    let doc = cursor.next().await.transpose()?;

    if let Some(doc) = doc {
        let doc = bson::from_bson::<T>(Bson::Document(doc))?;
        return Ok(Some(doc));
    }

    Ok(None)
}

pub async fn consume_cursor_and_convert<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<Document>,
) -> StdResult<Vec<T>, MongoDBError> {
    let mut documents: Vec<T> = Vec::new();

    while cursor.advance().await? {
        documents.push(
            bson::from_bson::<T>(Bson::Document(cursor.deserialize_current()?))?
        );
    }

    Ok(documents)
}

pub type PaginatedResult<T> = Result<(Vec<T>, u64)>;

#[async_trait]
trait CursorHelper
{
    // async fn consume(self) -> StdResult<Vec<Self>, MongoDBError>;
    // async fn consume_and_convert(self) -> StdResult<Vec<Self>, MongoDBError>;
    async fn convert_one_doc(self) -> StdResult<Option<Document>, MongoDBError>;
}

// #[async_trait]
// impl CursorHelper for Cursor<T>
// // where
// //     T: for<'de> Deserialize<'de> + Stream + StreamExt,
//  {

//     async fn convert_one_doc(self) -> StdResult<Option<Document>, MongoDBError> {
//         let doc = self.next().await.transpose()?;

//         if let Some(doc) = doc {
//             let doc = bson::from_bson::<T>(Bson::Document(doc))?;
//             return Ok(Some(doc));
//         }

//         Ok(None)
//     }
// }