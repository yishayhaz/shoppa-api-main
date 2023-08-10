use super::super::super::middlewares::CurrentUser;
use super::types;
use crate::{
    db::{AxumDBExtansion, OrderFunctions},
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::{doc, oid::ObjectId};
use shoppa_core::{
    db::{models::Order, Pagination},
    extractors::JsonWithValidation,
    ResponseBuilder,
};

pub async fn get_orders(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    pagination: Pagination,
    Query(query): Query<types::OrdersQuery>,
) -> HandlerResult {
    let orders = db
        .get_orders_for_store(
            Some(pagination),
            current_user.store_id,
            query.from,
            query.to,
            query.status,
            query.utm,
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

pub async fn update_order(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    Path(order_oid): Path<ObjectId>,
    JsonWithValidation(payload): JsonWithValidation<types::UpdateOrderStatusPayload>,
) -> HandlerResult {
    let filters = doc! {
        Order::fields().id: order_oid,
        Order::fields().parts(true).store: current_user.store_id,
    };

    let update = doc! {
        "$set": {
            "parts.$.status": payload.status.to_string(),
        }
    };

    let order = db.update_order(filters, update, None, None).await?;

    Ok(ResponseBuilder::success(Some(order), None, None).into_response())
}
