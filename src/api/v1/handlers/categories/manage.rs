use super::super::prelude::routes::*;
use crate::db::{inserts, inserts::InsertDocumentErrors};

pub async fn create_new_root_catagorie(db: DBExtension) -> HandlerResponse {
    Ok(().into_response())
}

pub async fn create_new_inner_catagorie(db: DBExtension) -> HandlerResponse {
    Ok(().into_response())
}

pub async fn create_new_inner_inner_catagorie(db: DBExtension) -> HandlerResponse {
    Ok(().into_response())
}
