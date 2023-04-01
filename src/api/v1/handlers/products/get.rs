use super::super::prelude::routes::*;
use super::types::CreateProductPayload;
use crate::db::{inserts, queries, inserts::InsertDocumentErrors};

pub async fn get_products(
    db: DBExtension,
) -> HandlerResponse {

    Ok(().into_response())

}
