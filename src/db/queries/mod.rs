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

use crate::prelude::*;
use futures_util::StreamExt;
use mongodb::{error::Error as MongoDBError, Cursor};
use serde::de::DeserializeOwned;
use serde::Deserialize;

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
        documents.push(bson::from_bson::<T>(Bson::Document(
            cursor.deserialize_current()?,
        ))?);
    }

    Ok(documents)
}

pub type PaginatedResult<T> = Result<(Vec<T>, u64)>;

#[async_trait]
pub trait CursorConsumer<T> {
    async fn consume(self) -> StdResult<Vec<T>, MongoDBError>;
}

#[async_trait]
pub trait CursorConverter<T> {
    async fn convert_one_doc(self) -> StdResult<Option<T>, MongoDBError>;
    async fn consume_and_convert(self) -> StdResult<Vec<T>, MongoDBError>;
}

#[async_trait]
pub trait CursorExtractors {
    async fn extract_count(self) -> Result<u64>;
}

#[async_trait]
impl<T> CursorConsumer<T> for Cursor<T>
where
    T: DeserializeOwned + Sync + Send + Unpin,
{
    async fn consume(mut self) -> StdResult<Vec<T>, MongoDBError> {
        let mut documents: Vec<T> = Vec::new();

        while self.advance().await? {
            documents.push(self.deserialize_current()?);
        }

        Ok(documents)
    }
}

#[async_trait]
impl<T> CursorConverter<T> for Cursor<Document>
where
    T: for<'a> Deserialize<'a> + Sync + Send,
{
    async fn convert_one_doc(mut self) -> StdResult<Option<T>, MongoDBError> {
        let doc = self.next().await.transpose()?;

        if let Some(doc) = doc {
            let doc = bson::from_bson::<T>(Bson::Document(doc))?;
            return Ok(Some(doc));
        }

        Ok(None)
    }
    async fn consume_and_convert(mut self) -> StdResult<Vec<T>, MongoDBError> {
        let mut documents: Vec<T> = Vec::new();

        while self.advance().await? {
            documents.push(bson::from_bson::<T>(Bson::Document(
                self.deserialize_current()?,
            ))?);
        }

        Ok(documents)
    }
}

#[async_trait]
impl CursorExtractors for Cursor<Document> {
    // TODO improve error handling
    async fn extract_count(mut self) -> Result<u64> {
        let doc = self
            .next()
            .await
            .transpose()
            .map_err(|e| Error::DBError(("unknown", MongoDBError::from(e))))?;

        if let Some(doc) = doc {
            let count = doc.get_i32("count").unwrap_or(0) as u64;
            return Ok(count);
        }

        Ok(0)
    }
}
