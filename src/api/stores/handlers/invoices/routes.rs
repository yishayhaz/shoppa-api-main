use super::types;
use crate::{
    api::stores::middlewares::CurrentUser,
    db::{AxumDBExtansion, InvoicesFunctions, StoreUserStoreFunctions},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::{doc, oid::ObjectId};
use shoppa_core::{
    db::{
        models::{FileDocument, FileTypes},
        Pagination,
    },
    extractors::{JsonWithValidation, MultipartFormWithValidation},
    parser::FieldPatch,
    ResponseBuilder,
};

pub async fn get_invoices(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    pagination: Pagination,
    Query(query): Query<types::InvoicesQuery>,
) -> HandlerResult {
    let invoices = db
        .get_invoices_for_external(
            Some(pagination),
            Some(current_user.store_id),
            query.from,
            query.to,
            query.invoice_type,
            None,
        )
        .await?;

    Ok(ResponseBuilder::paginated_response(&invoices).into_response())
}
