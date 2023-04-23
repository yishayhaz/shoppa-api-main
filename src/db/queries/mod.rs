mod categories;
mod prelude;
mod products;
mod store;
mod users;
mod variants;
mod contact_us;

pub use categories::*;
pub use products::*;
pub use store::*;
pub use users::*;
pub use variants::*;
pub use contact_us::*;

use mongodb::{error::Error, Cursor};
use serde::Deserialize;
use axum::response::Response;

pub async fn consume_cursor<T: for<'a> Deserialize<'a>>(
    mut cursor: Cursor<T>,
) -> Result<Vec<T>, Error> {
    let mut documents: Vec<T> = Vec::new();

    while cursor.advance().await? {
        documents.push(cursor.deserialize_current()?);
    }

    Ok(documents)
}


pub type PaginatedResult<T> = Result<(Vec<T>, u64), Response>;