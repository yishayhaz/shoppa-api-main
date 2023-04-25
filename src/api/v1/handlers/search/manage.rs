use crate::{
    api::v1::middlewares::*,
    prelude::{handlers::*, *},
};

pub async fn search_products(db: DBExtension, _: OnlyInDev) -> HandlerResult {
    Ok(ResponseBuilder::success(Some("products"), None, None).into_response())
}

pub async fn search_stores(db: DBExtension, _: OnlyInDev) -> HandlerResult {
    Ok(ResponseBuilder::success(Some("stores"), None, None).into_response())
}
