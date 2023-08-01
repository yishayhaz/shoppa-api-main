use super::{
    super::super::middlewares::CurrentUser,
    types::{
        CreateProductPayload, EditProductPayload, GetProductsQueryParams, UploadProductAssetPayload,
    },
};
use crate::{
    db::{AxumDBExtansion, ProductSortBy, StoreProductFunctions},
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
        models::{EmbeddedDocument, FileDocument, FileTypes, Product, ProductStatus, Order},
        OptionalSorter, Pagination,
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    ResponseBuilder,
};

pub async fn get_orders(db: AxumDBExtansion,
    current_user: CurrentUser, pagination: Pagination) {

    let orders = db.aggregate_orders(vec![], None, None).await;

    Ok(ResponseBuilder::paginated_response(&orders).into_response())

}

pub async fn get_order(db: AxumDBExtansion, current_user: CurrentUser, Path(order_oid): Path<ObjectId>) {  
    let order = db.get_order_by_id(&order_oid, None, None, None).await?;

    Ok(ResponseBuilder::success(Some(order), None, None).into_response())
}

pub async fn update_order_status() {
    todo!()
}