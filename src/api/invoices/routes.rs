use crate::{db::AxumDBExtansion, prelude::*};
use axum::{extract::Path, response::IntoResponse};
use bson::{doc, oid::ObjectId};
use shoppa_core::ResponseBuilder;

pub async fn update_mail_sent(
    db: AxumDBExtansion,
    Path((token, order_id)): Path<(String, ObjectId)>,
) -> HandlerResult {
    if token.is_empty() || token != "token" {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("hello stranger 👋"), Some(404))
                .into_response(),
        );
    }

    let filter = doc! {"order_id": order_id};

    let invoice = db.get_invoices(filter, None, None, None).await?;

    if invoice.is_empty() {
        return Ok(
            ResponseBuilder::<u16>::error("", None, Some("Invoice not found"), Some(404))
                .into_response(),
        );
    }

    // loop through invoices and update mail_sent to true

    Ok(ResponseBuilder::success(Some(invoice), None, Some(200)).into_response())
}
