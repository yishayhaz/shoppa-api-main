use super::types;
use crate::{
    db::{queries, models::ProductSortBy},
    prelude::{handlers::*, *},
    api::v1::middlewares::*,
};
use axum::extract::MatchedPath;

pub async fn get_products(
    db: DBExtension,
    path: MatchedPath,
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
        query.category,
        path.as_str().ends_with("/infinite")
    )
    .await?;

    Ok(ResponseBuilder::paginated_response(&products).into_response())
}
