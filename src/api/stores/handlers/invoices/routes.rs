use super::types;
use crate::{
    api::stores::middlewares::CurrentUser,
    db::{AxumDBExtansion, InvoicesFunctions},
    helpers::types::AxumStorgeClientExtension,
    prelude::*,
};
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
};
use bson::oid::ObjectId;
use shoppa_core::{db::Pagination, file_storage::Buckets, ResponseBuilder};

pub async fn get_invoices(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    pagination: Pagination,
    Query(query): Query<types::InvoicesQuery>,
) -> HandlerResult {
    let invoices = db
        .get_invoices_for_external(
            Some(pagination),
            current_user.store_id,
            query.from,
            query.to,
            query.invoice_type,
            None,
        )
        .await?;

    Ok(ResponseBuilder::paginated_response(&invoices).into_response())
}

pub async fn install_invoice(
    db: AxumDBExtansion,
    current_user: CurrentUser,
    storage_client: AxumStorgeClientExtension,
    Path(invoice_oid): Path<ObjectId>,
) -> HandlerResult {
    let invoice = db.get_invoice_by_id(&invoice_oid, None, None, None).await?;

    if invoice.is_none() {
        return Ok(
            ResponseBuilder::<()>::error("Invoice not found", None, None, Some(404))
                .into_response(),
        );
    }

    let invoice = invoice.unwrap();

    if invoice.store.doc_id() != current_user.store_id {
        return Ok(
            ResponseBuilder::<()>::error("Invoice not found", None, None, Some(404))
                .into_response(),
        );
    }

    let url = storage_client
        .generate_download_url(invoice.copy.path.as_str(), 60 * 60 * 12, Buckets::Invoice)
        .await;
    if let Ok(url) = url {
        return Ok(ResponseBuilder::success(Some(url), None, None).into_response());
    }

    Ok(
        ResponseBuilder::<()>::error("Failed to generate download URL", None, None, Some(500))
            .into_response(),
    )
}
