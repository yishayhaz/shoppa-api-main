use super::{super::prelude::routes::*, types};
use crate::db::{inserts, queries};
use axum::http::StatusCode;

pub async fn search_products(
    db: DBExtension,
    _: OnlyInDev,
) -> HandlerResponse {
    Ok(ResponseBuilder::success(Some("products"), None, None).into_response())
}

pub async fn search_stores(
    db: DBExtension,
    _: OnlyInDev,
) -> HandlerResponse {
    Ok(ResponseBuilder::success(Some("stores"), None, None).into_response())
}
