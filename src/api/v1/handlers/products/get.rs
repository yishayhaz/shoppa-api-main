use super::super::prelude::routes::*;
use super::types;
use crate::db::queries;

pub async fn get_products(
    db: DBExtension,
    pagination: Pagination,
    sorting: OptionalSorting,
    Query(query): Query<types::GetProductQueryParams>,
) -> HandlerResponse {
    let products = queries::get_products_for_extarnel(
        &db,
        Some(pagination),
        sorting.into_option(),
        query.free_text,
        query.store_id,
    )
    .await?;

    Ok(ResponseBuilder::success(Some(products), None, None).into_response())
}
