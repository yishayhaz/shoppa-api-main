mod categories;
mod contact_us;
mod prelude;
mod products;
mod store;
mod users;
mod variants;
mod analytics;

use bson::{Bson, Document};
pub use categories::*;
pub use contact_us::*;
pub use products::*;
pub use store::*;
pub use users::*;
pub use variants::*;
pub use analytics::*;

use crate::prelude::*;
use futures_util::StreamExt;
use mongodb::Cursor;
use serde::de::DeserializeOwned;
use serde::Deserialize;

pub type PaginatedResult<T> = Result<(Vec<T>, u64)>;

#[async_trait]
pub trait CursorConsumer<T> {
    async fn consume(self) -> Result<Vec<T>>;
}

#[async_trait]
pub trait CursorConverter {
    async fn convert_one_doc<T: for<'a> Deserialize<'a>>(self) -> Result<Option<T>>;
    async fn consume_and_convert<T: for<'a> Deserialize<'a> + Send + Sync>(self) -> Result<Vec<T>>;
    async fn extract_count(self) -> Result<u64>;
}

#[async_trait]
impl<T> CursorConsumer<T> for Cursor<T>
where
    T: DeserializeOwned + Sync + Send + Unpin,
{
    async fn consume(mut self) -> Result<Vec<T>> {
        let mut documents: Vec<T> = Vec::new();

        while self
            .advance()
            .await
            .map_err(|e| Error::DBError(("unknown", e)))?
        {
            documents.push(
                self.deserialize_current()
                    .map_err(|e| Error::DBError(("unknown", e)))?,
            );
        }

        Ok(documents)
    }
}

#[async_trait]
impl CursorConverter for Cursor<Document> {
    async fn convert_one_doc<T: for<'a> Deserialize<'a>>(mut self) -> Result<Option<T>> {
        let doc = self
            .next()
            .await
            .transpose()
            .map_err(|e| Error::DBError(("unknown", e)))?;

        if let Some(doc) = doc {
            let doc = bson::from_bson::<T>(Bson::Document(doc))
                .map_err(|e| Error::DBError(("unknown", e.into())))?;
            return Ok(Some(doc));
        }

        Ok(None)
    }
    async fn consume_and_convert<T: for<'a> Deserialize<'a> + Send + Sync>(
        mut self,
    ) -> Result<Vec<T>> {
        let mut documents: Vec<T> = Vec::new();

        while self
            .advance()
            .await
            .map_err(|e| Error::DBError(("unknown", e)))?
        {
            documents.push(
                bson::from_bson::<T>(Bson::Document(
                    self.deserialize_current()
                        .map_err(|e| Error::DBError(("faild deserialize data from db", e)))?,
                ))
                .map_err(|e| Error::DBError(("faild converting doc into Bson::Document", e.into())))?,
            );
        }

        Ok(documents)
    }

    async fn extract_count(mut self) -> Result<u64> {
        let doc = self
            .next()
            .await
            .transpose()
            .map_err(|e| Error::DBError(("unknown - when extracting count", e)))?;

        if let Some(doc) = doc {
            let count = doc.get_i32("count").unwrap_or(0) as u64;
            return Ok(count);
        }

        Ok(0)
    }
}
