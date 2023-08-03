use super::super::super::middlewares::CurrentUser;
use crate::{
    db::{AxumDBExtansion, OrderFunctions, ProductSortBy, StoreProductFunctions},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{
    db::{
        models::{EmbeddedDocument, FileDocument, FileTypes, Order, Product, ProductStatus},
        OptionalSorter, Pagination,
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn get_orders(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    pagination: Pagination,
) -> HandlerResult {
    let orders = db
        .get_orders_for_store(
            Some(pagination),
            current_user.store_id,
            None,
            None,
            None,
            None,
        )
        .await?;

    Ok(ResponseBuilder::paginated_response(&orders).into_response())
}

pub async fn get_order(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(order_oid): Path<ObjectId>,
) -> HandlerResult {
    let order = db
        .get_order_by_id_for_store(current_user.store_id, order_oid, None)
        .await?;

    Ok(ResponseBuilder::success(Some(order), None, None).into_response())
}
