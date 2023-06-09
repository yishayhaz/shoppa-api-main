use super::types;
use crate::{
    db::{AxumDBExtansion, ProductFunctions, ProductSortBy},
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{OptionalSorter, Pagination},
    ResponseBuilder,
};

pub async fn get_products_infinite(
    db: AxumDBExtansion,
    pagination: Pagination,
    Query(query): Query<types::GetProductsInfiniteQueryParams>,
) -> HandlerResult {
    todo!()
}

pub async fn get_products(
    db: AxumDBExtansion,
    pagination: Pagination,
    sorting: OptionalSorter<ProductSortBy>,
    Query(query): Query<types::GetProductQueryParams>,
) -> HandlerResult {
    let products = db
        .get_products_for_extarnel(
            Some(pagination),
            sorting.0,
            query.free_text,
            query.store_id,
            query.category_id,
            None,
        )
        .await?;

    Ok(ResponseBuilder::paginated_response(&products).into_response())
}

pub async fn get_product(db: AxumDBExtansion, Path(product_id): Path<ObjectId>) -> HandlerResult {
    let product = db.get_one_product_for_extarnel(&product_id, None).await?;

    if product.is_none() {
        return Ok(ResponseBuilder::error("", Some(""), None, Some(404)).into_response());
    }

    Ok(ResponseBuilder::success(product, None, None).into_response())
}

pub async fn products_autocomplete(
    db: AxumDBExtansion,
    Query(query): Query<types::GetProductsAutoCompleteQueryParams>,
) -> HandlerResult {
    // maybe get random products if there is no free text
    if query.free_text.is_none() {
        return Ok(ResponseBuilder::<Vec<u16>>::success(Some(vec![]), None, None).into_response());
    }
    todo!()
    // let products = queries::get_products_names_for_autocomplete(
    //     &db,
    //     query.free_text.unwrap(),
    //     query.store_id,
    //     query.category_id,
    // )
    // .await?;

    // Ok(ResponseBuilder::success(Some(products), None, None).into_response())
}

pub async fn products_count(
    db: AxumDBExtansion,
    Query(query): Query<types::GetProductsCountQueryParams>,
) -> HandlerResult {
    todo!()
    // let count = queries::get_products_count(&db, query.store_id, query.category_id).await?;

    // Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}

pub async fn add_view_to_product(
    db: AxumDBExtansion,
    Path(product_id): Path<ObjectId>,
) -> HandlerResult {
    let product = db.add_view_to_product(&product_id, None).await?;

    if let Some(product) = product {
        // the views is being returend before the update, so we need to add 1 to the views
        return Ok(
            ResponseBuilder::success(Some(product.analytics.views + 1), None, None).into_response(),
        );
    }

    Ok(ResponseBuilder::error("", Some(""), None, Some(404)).into_response())
}
