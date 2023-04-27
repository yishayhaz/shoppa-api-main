use super::types;
use crate::{
    db::{queries, models::ProductSortBy},
    prelude::{handlers::*, *},
    api::v1::middlewares::*,
};

pub async fn get_products(
    db: DBExtension,
    pagination: Pagination,
    sorting: OptionalSorting<ProductSortBy>,
    Query(query): Query<types::GetProductQueryParams>,
) -> HandlerResult {
    let products = queries::get_products_for_extarnel(
        &db,
        Some(pagination),
        sorting.into(),
        query.free_text,
        query.store_id,
    )
    .await?;

    Ok(ResponseBuilder::paginated_response(&products).into_response())
}
