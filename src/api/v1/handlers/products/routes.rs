use super::types;
use crate::{
    api::v1::middlewares::*,
    db::{models::ProductSortBy, queries, updates},
    prelude::{handlers::*, *},
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
        path.as_str().ends_with("/infinite"),
    )
    .await?;

    Ok(ResponseBuilder::paginated_response(&products).into_response())
}

pub async fn get_product(db: DBExtension, Path(product_id): Path<ObjectId>) -> HandlerResult {
    let product = queries::get_one_product_for_extarnel(&db, &product_id).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::not_found_error("products", &product_id).into_response());
    }

    Ok(ResponseBuilder::success(product, None, None).into_response())
}

pub async fn products_names_for_autocomplete(
    db: DBExtension,
    Query(query): Query<types::GetProductQueryParams>,
) -> HandlerResult {
    if query.free_text.is_none() {
        return Ok(ResponseBuilder::<Vec<u16>>::success(Some(vec![]), None, None).into_response());
    }

    let products = queries::get_products_names_for_autocomplete(
        &db,
        query.free_text.unwrap(),
        query.store_id,
        query.category_id,
    )
    .await?;

    Ok(ResponseBuilder::success(Some(products), None, None).into_response())
}

pub async fn products_count(
    db: DBExtension,
    Query(query): Query<types::GetProductsCountQueryParams>,
) -> HandlerResult {
    let count = queries::get_products_count(&db, query.store_id, query.category_id).await?;

    Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}


pub async fn add_view_to_product(
    db: DBExtension,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    let product = updates::add_view_to_product(&db, &product_id, None).await?;

    if let Some(product) = product {
        // the views is being returend before the update, so we need to add 1 to the views
        return Ok(
            ResponseBuilder::success(Some(product.analytics.views + 1), None, None).into_response(),
        );
    }

    Ok(ResponseBuilder::not_found_error("product", &product_id).into_response())
}
