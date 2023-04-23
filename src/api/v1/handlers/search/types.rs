use super::super::prelude::types::*;

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ProductSearchPayload {
    pub query: String,
    pub sort: Option<String>, // TODO Make this an enum, "popularity", "price", "date"
}

pub struct _StoreSearchPayload {
    pub query: String,
}
