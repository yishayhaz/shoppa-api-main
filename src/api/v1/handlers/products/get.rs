use super::super::prelude::routes::*;
use super::types;
use crate::db::queries;


pub async fn get_products(
    db: DBExtension,
    pagination: Pagination,
    Query(query): Query<types::GetProductQueryParams>
) -> HandlerResponse {
    let products = queries::get_products_for_extarnel(&db, query.free_text, Some(pagination)).await?;

    Ok(ResponseBuilder::success(Some(products), None, None).into_response())
}
