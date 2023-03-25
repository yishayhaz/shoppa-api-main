use crate::{
    db::{inserts, inserts::InsertDocumentErrors},
    helpers::{
        types::{DBExtension, HandlerResponse, ResponseBuilder},
        json::JsonWithValidation
    },
};
use axum::{response::IntoResponse, Json};
use tower_cookies::Cookies;

pub async fn create_new_root_catagorie(
    db: DBExtension,
) -> HandlerResponse {

    Ok(().into_response())
}

pub async fn create_new_inner_catagorie(
    db: DBExtension,
) -> HandlerResponse {
    
    Ok(().into_response())
}

pub async fn create_new_inner_inner_catagorie(
    db: DBExtension,
) -> HandlerResponse {
    
    Ok(().into_response())
}