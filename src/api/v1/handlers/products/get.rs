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
        query.category_id,
        path.as_str().ends_with("/infinite")
    )
    .await?;

    Ok(ResponseBuilder::paginated_response(&products).into_response())
}


pub async fn get_product(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    
    let product = queries::get_one_product_for_extarnel(&db, &product_id).await?;

    if product.is_none(){
        return Ok(ResponseBuilder::not_found_error("products", &product_id).into_response());
    }

    Ok(ResponseBuilder::success(product, None, None).into_response())
}
