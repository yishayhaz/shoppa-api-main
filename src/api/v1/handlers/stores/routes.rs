use super::types;
use crate::{
    db::queries,
    prelude::{handlers::*, *},
};

pub async fn get_stores_autocomplete(
    db: DBExtension,
    Query(query): Query<types::SearchStoresQueryParams>,
) -> HandlerResult {
    if query.free_text.is_none() {
        return Ok(ResponseBuilder::<Vec<u16>>::success(Some(vec![]), None, None).into_response());
    }

    let stores = queries::get_stores_names_for_autocomplete(&db, query.free_text.unwrap()).await?;

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_stores_count(db: DBExtension) -> HandlerResult {
    let count = queries::get_stores_count(&db).await?;

    Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}

pub async fn get_stores(db: DBExtension) -> HandlerResult {
    let stores = queries::get_stores(&db).await?;

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}
