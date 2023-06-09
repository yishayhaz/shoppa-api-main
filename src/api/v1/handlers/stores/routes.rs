use super::types;
use crate::{
    db::{AxumDBExtansion, StoreFunctions},
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{db::Pagination, ResponseBuilder};

pub async fn get_stores_autocomplete(
    db: AxumDBExtansion,
    Query(query): Query<types::SearchStoresQueryParams>,
) -> HandlerResult {
    let stores;

    if let Some(free_text) = query.free_text {
        stores = db
            .get_stores_names_for_autocomplete(free_text, None)
            .await?;
    } else {
        stores = db.get_random_stores_names(None).await?;
    }

    Ok(ResponseBuilder::success(Some(stores), None, None).into_response())
}

pub async fn get_stores_count(db: AxumDBExtansion) -> HandlerResult {
    let count = db.count_stores(None, None).await?;

    Ok(ResponseBuilder::success(Some(count), None, None).into_response())
}

pub async fn get_stores(
    db: AxumDBExtansion,
    pagination: Pagination,
    Query(query): Query<types::SearchStoresQueryParams>,
) -> HandlerResult {
    let stores = db
        .get_many_stores_for_extarnel(Some(pagination), query.free_text, None)
        .await?;

    Ok(ResponseBuilder::paginated_response(&stores).into_response())
}

pub async fn get_store_by_id(db: AxumDBExtansion, Path(store_id): Path<ObjectId>) -> HandlerResult {
    let store = db.get_store_for_extarnel(&store_id, None).await?;

    if store.is_none() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("store not found"), Some(400))
                .into_response(),
        );
    }

    Ok(ResponseBuilder::success(store, None, None).into_response())
}
