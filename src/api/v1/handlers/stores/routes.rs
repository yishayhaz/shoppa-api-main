use super::types;
use crate::{
    db::queries,
    prelude::{handlers::*, *},
};

pub async fn get_stores_autocomplete(
    db: DBExtension,
    Query(query): Query<types::SearchStoresQueryParams>,
) -> HandlerResult {
    let stores;

    if query.free_text.is_none() {
        stores = queries::get_random_stores_names(&db).await?;
    } else {
        stores = queries::get_stores_names_for_autocomplete(&db, query.free_text.unwrap()).await?;
    }

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_stores_count(db: DBExtension) -> HandlerResult {
    let count = queries::get_stores_count(&db).await?;

    Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}

pub async fn get_stores(
    db: DBExtension,
    pagination: Pagination,
    Query(query): Query<types::SearchStoresQueryParams>,
) -> HandlerResult {
    let stores = queries::get_stores_for_extarnel(&db, Some(pagination), query.free_text).await?;

    Ok(ResponseBuilder::paginated_response(&stores).into_response())
}
